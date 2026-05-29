use crate::{
    commands::{CommandPlan, CommandRisk, CommandSpec, PermissionGate, registry},
    database::{CommandAuditEvent, DatabaseError, ProjectDatabase},
    dataset::{DatasetError, import_image_folder},
    duplicates::{DuplicateError, audit_exact_duplicates},
    settings::{AppSettings, SettingsError},
    workspace::{ProjectManifest, WorkspaceError},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, path::PathBuf};

#[derive(Clone, Debug)]
pub struct AppState {
    pub project_root: Option<PathBuf>,
    pub active_project: Option<ProjectManifest>,
    pub commands: Vec<CommandSpec>,
    pub settings: AppSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionMode {
    DryRun,
    Apply,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandOutcome {
    pub applied: bool,
    pub summary: String,
    pub artifacts: Vec<PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            project_root: None,
            active_project: None,
            commands: registry(),
            settings: AppSettings::default(),
        }
    }

    pub fn execute(
        &mut self,
        plan: CommandPlan,
        mode: ExecutionMode,
    ) -> Result<CommandOutcome, AppStateError> {
        match plan {
            CommandPlan::CreateProject {
                root,
                name,
                template,
            } => {
                let manifest = ProjectManifest::new(name, template);

                if mode == ExecutionMode::DryRun {
                    return Ok(CommandOutcome {
                        applied: false,
                        summary: format!("Would create project at {}", root.display()),
                        artifacts: manifest.artifact_paths(&root),
                    });
                }

                let manifest_path = manifest.save(&root)?;
                self.project_root = Some(root);
                self.active_project = Some(manifest);

                Ok(CommandOutcome {
                    applied: true,
                    summary: format!("Created project manifest at {}", manifest_path.display()),
                    artifacts: vec![manifest_path],
                })
            }
            CommandPlan::OpenProject { root } => {
                let manifest = ProjectManifest::load(&root)?;
                self.settings.remember_project(root.clone());

                if mode == ExecutionMode::DryRun {
                    return Ok(CommandOutcome {
                        applied: false,
                        summary: format!("Would open project at {}", root.display()),
                        artifacts: vec![root.join(crate::workspace::PROJECT_FILE_NAME)],
                    });
                }

                self.project_root = Some(root.clone());
                self.active_project = Some(manifest);

                Ok(CommandOutcome {
                    applied: true,
                    summary: format!("Opened project at {}", root.display()),
                    artifacts: vec![root.join(crate::workspace::PROJECT_FILE_NAME)],
                })
            }
            CommandPlan::ImportDataset { path } => {
                let (Some(project_root), Some(project)) =
                    (self.project_root.as_ref(), self.active_project.as_ref())
                else {
                    return Err(AppStateError::NoActiveProject);
                };

                if mode == ExecutionMode::DryRun {
                    return Ok(CommandOutcome {
                        applied: false,
                        summary: format!("Would import image metadata from {}", path.display()),
                        artifacts: vec![
                            project_root
                                .join(&project.artifacts.metadata_dir)
                                .join(crate::dataset::IMAGES_CSV_NAME),
                        ],
                    });
                }

                let summary = import_image_folder(path, project_root, project)?;

                Ok(CommandOutcome {
                    applied: true,
                    summary: format!(
                        "Imported {} readable images from {} candidate files",
                        summary.readable_images, summary.total_files
                    ),
                    artifacts: vec![summary.output_csv],
                })
            }
            CommandPlan::AuditDuplicates { path } => {
                let (Some(project_root), Some(project)) =
                    (self.project_root.as_ref(), self.active_project.as_ref())
                else {
                    return Err(AppStateError::NoActiveProject);
                };

                if mode == ExecutionMode::DryRun {
                    return Ok(CommandOutcome {
                        applied: false,
                        summary: format!("Would audit exact duplicates in {}", path.display()),
                        artifacts: vec![
                            project_root
                                .join(&project.artifacts.metadata_dir)
                                .join(crate::duplicates::EXACT_DUPLICATES_CSV_NAME),
                        ],
                    });
                }

                let summary = audit_exact_duplicates(path, project_root, project)?;

                Ok(CommandOutcome {
                    applied: true,
                    summary: format!(
                        "Found {} exact duplicate groups and {} duplicate files beyond first copies",
                        summary.duplicate_groups, summary.duplicate_files_beyond_first
                    ),
                    artifacts: vec![summary.output_csv],
                })
            }
            CommandPlan::GenerateSplits { seed } => Ok(not_implemented_outcome(
                "Generate splits",
                format!("Split generation is planned with seed {seed}."),
            )),
            CommandPlan::ImportPredictions { path } => Ok(not_implemented_outcome(
                "Import predictions",
                format!("Prediction import is planned for {path}."),
            )),
            CommandPlan::EvaluateMetrics => Ok(not_implemented_outcome(
                "Evaluate metrics",
                "Metric evaluation is planned.".to_string(),
            )),
            CommandPlan::ExportReport => Ok(not_implemented_outcome(
                "Export report",
                "Report export is planned.".to_string(),
            )),
        }
    }

    pub fn execute_with_audit(
        &mut self,
        actor: impl Into<String>,
        plan: CommandPlan,
        mode: ExecutionMode,
    ) -> Result<CommandOutcome, AppStateError> {
        let outcome = self.execute(plan.clone(), mode)?;

        if outcome.applied
            && let Some(project_root) = self.project_root.as_ref()
        {
            let database = ProjectDatabase::open(project_root)?;
            database.insert_command_event(&CommandAuditEvent::new(
                actor,
                mode,
                plan,
                outcome.clone(),
            ))?;
        }

        Ok(outcome)
    }

    pub fn approval_required_count(&self) -> usize {
        self.commands
            .iter()
            .filter(|command| command.permission != PermissionGate::None)
            .count()
    }

    pub fn expensive_command_count(&self) -> usize {
        self.commands
            .iter()
            .filter(|command| command.risk == CommandRisk::Expensive)
            .count()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn not_implemented_outcome(label: &str, summary: String) -> CommandOutcome {
    CommandOutcome {
        applied: false,
        summary: format!("{label}: {summary}"),
        artifacts: Vec::new(),
    }
}

#[derive(Debug)]
pub enum AppStateError {
    NoActiveProject,
    Workspace(WorkspaceError),
    Settings(SettingsError),
    Dataset(DatasetError),
    Database(DatabaseError),
    Duplicate(DuplicateError),
}

impl From<WorkspaceError> for AppStateError {
    fn from(error: WorkspaceError) -> Self {
        Self::Workspace(error)
    }
}

impl From<SettingsError> for AppStateError {
    fn from(error: SettingsError) -> Self {
        Self::Settings(error)
    }
}

impl From<DatasetError> for AppStateError {
    fn from(error: DatasetError) -> Self {
        Self::Dataset(error)
    }
}

impl From<DatabaseError> for AppStateError {
    fn from(error: DatabaseError) -> Self {
        Self::Database(error)
    }
}

impl From<DuplicateError> for AppStateError {
    fn from(error: DuplicateError) -> Self {
        Self::Duplicate(error)
    }
}

impl fmt::Display for AppStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoActiveProject => write!(formatter, "no active project"),
            Self::Workspace(error) => write!(formatter, "{error}"),
            Self::Settings(error) => write!(formatter, "{error}"),
            Self::Dataset(error) => write!(formatter, "{error}"),
            Self::Database(error) => write!(formatter, "{error}"),
            Self::Duplicate(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for AppStateError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::CommandPlan,
        workspace::{PROJECT_FILE_NAME, ResearchTemplate},
    };

    #[test]
    fn dry_run_create_project_does_not_write_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("dry-run-project");
        let mut state = AppState::new();

        let outcome = state
            .execute(
                CommandPlan::CreateProject {
                    root: root.clone(),
                    name: "Dry run".to_string(),
                    template: ResearchTemplate::GenericImageClassification,
                },
                ExecutionMode::DryRun,
            )
            .expect("dry run");

        assert!(!outcome.applied);
        assert!(
            outcome
                .artifacts
                .iter()
                .any(|path| path.ends_with("metadata"))
        );
        assert!(!root.exists());
        assert!(state.active_project.is_none());
    }

    #[test]
    fn apply_create_project_writes_manifest_and_tracks_state() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("leaf-study");
        let mut state = AppState::new();

        let outcome = state
            .execute(
                CommandPlan::CreateProject {
                    root: root.clone(),
                    name: "Leaf study".to_string(),
                    template: ResearchTemplate::LeafDiseaseClassification,
                },
                ExecutionMode::Apply,
            )
            .expect("create project");

        assert!(outcome.applied);
        assert_eq!(state.project_root.as_deref(), Some(root.as_path()));
        assert_eq!(
            state
                .active_project
                .as_ref()
                .map(|project| project.project.name.as_str()),
            Some("Leaf study")
        );
        assert!(root.join(PROJECT_FILE_NAME).is_file());
    }

    #[test]
    fn open_project_loads_manifest_and_tracks_recent_project() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("existing-study");
        ProjectManifest::new("Existing", ResearchTemplate::BinaryMedicalImaging)
            .save(&root)
            .expect("save fixture project");
        let mut state = AppState::new();

        let outcome = state
            .execute(
                CommandPlan::OpenProject { root: root.clone() },
                ExecutionMode::Apply,
            )
            .expect("open project");

        assert!(outcome.applied);
        assert_eq!(state.project_root.as_deref(), Some(root.as_path()));
        assert_eq!(state.settings.last_project.as_deref(), Some(root.as_path()));
        assert_eq!(
            state
                .active_project
                .as_ref()
                .map(|project| project.project.template),
            Some(ResearchTemplate::BinaryMedicalImaging)
        );
    }

    #[test]
    fn import_dataset_requires_active_project() {
        let temp = tempfile::tempdir().expect("tempdir");
        let mut state = AppState::new();

        let error = state
            .execute(
                CommandPlan::ImportDataset {
                    path: temp.path().to_path_buf(),
                },
                ExecutionMode::DryRun,
            )
            .expect_err("missing active project should fail");

        assert!(matches!(error, AppStateError::NoActiveProject));
    }

    #[test]
    fn execute_with_audit_records_applied_commands() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("audited-project");
        let mut state = AppState::new();

        state
            .execute_with_audit(
                "test",
                CommandPlan::CreateProject {
                    root: root.clone(),
                    name: "Audited".to_string(),
                    template: ResearchTemplate::GenericImageClassification,
                },
                ExecutionMode::Apply,
            )
            .expect("create project");

        let database = ProjectDatabase::open(&root).expect("database");
        assert_eq!(database.command_event_count().expect("event count"), 1);
    }
}
