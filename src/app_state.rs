use crate::{
    commands::{CommandPlan, CommandRisk, CommandSpec, PermissionGate, registry},
    workspace::{ProjectManifest, WorkspaceError},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, path::PathBuf};

#[derive(Clone, Debug)]
pub struct AppState {
    pub project_root: Option<PathBuf>,
    pub active_project: Option<ProjectManifest>,
    pub commands: Vec<CommandSpec>,
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
            CommandPlan::ImportDataset { path } => Ok(not_implemented_outcome(
                "Import dataset",
                format!("Dataset import is planned for {path}."),
            )),
            CommandPlan::AuditDuplicates => Ok(not_implemented_outcome(
                "Audit duplicates",
                "Duplicate auditing is planned.".to_string(),
            )),
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
    Workspace(WorkspaceError),
}

impl From<WorkspaceError> for AppStateError {
    fn from(error: WorkspaceError) -> Self {
        Self::Workspace(error)
    }
}

impl fmt::Display for AppStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workspace(error) => write!(formatter, "{error}"),
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
}
