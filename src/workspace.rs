use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

pub const PROJECT_FILE_NAME: &str = "vision-workbench.toml";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub schema_version: u32,
    pub project: ProjectInfo,
    pub artifacts: ArtifactLayout,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub template: ResearchTemplate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchTemplate {
    GenericImageClassification,
    BinaryMedicalImaging,
    LeafDiseaseClassification,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLayout {
    pub metadata_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub predictions_dir: PathBuf,
    pub figures_dir: PathBuf,
    pub manuscripts_dir: PathBuf,
    pub agents_dir: PathBuf,
}

impl ProjectManifest {
    pub fn new(name: impl Into<String>, template: ResearchTemplate) -> Self {
        Self {
            schema_version: 1,
            project: ProjectInfo {
                name: name.into(),
                template,
            },
            artifacts: ArtifactLayout::default(),
        }
    }

    pub fn save(&self, root: impl AsRef<Path>) -> Result<PathBuf, WorkspaceError> {
        let root = root.as_ref();
        fs::create_dir_all(root)?;
        self.artifacts.create_dirs(root)?;

        let path = root.join(PROJECT_FILE_NAME);
        let encoded = toml::to_string_pretty(self)?;
        fs::write(&path, encoded)?;
        Ok(path)
    }

    pub fn load(root: impl AsRef<Path>) -> Result<Self, WorkspaceError> {
        let path = root.as_ref().join(PROJECT_FILE_NAME);
        let encoded = fs::read_to_string(path)?;
        Ok(toml::from_str(&encoded)?)
    }

    pub fn artifact_paths(&self, root: impl AsRef<Path>) -> Vec<PathBuf> {
        self.artifacts.paths(root.as_ref())
    }
}

impl Default for ArtifactLayout {
    fn default() -> Self {
        Self {
            metadata_dir: "metadata".into(),
            reports_dir: "reports".into(),
            predictions_dir: "reports/predictions".into(),
            figures_dir: "reports/figures".into(),
            manuscripts_dir: "manuscript".into(),
            agents_dir: ".vision-workbench/agents".into(),
        }
    }
}

impl ArtifactLayout {
    pub fn paths(&self, root: &Path) -> Vec<PathBuf> {
        [
            &self.metadata_dir,
            &self.reports_dir,
            &self.predictions_dir,
            &self.figures_dir,
            &self.manuscripts_dir,
            &self.agents_dir,
        ]
        .into_iter()
        .map(|directory| root.join(directory))
        .collect()
    }

    pub fn create_dirs(&self, root: &Path) -> io::Result<()> {
        for directory in self.paths(root) {
            fs::create_dir_all(directory)?;
        }

        Ok(())
    }
}

impl ResearchTemplate {
    pub fn label(self) -> &'static str {
        match self {
            Self::GenericImageClassification => "Generic image classification",
            Self::BinaryMedicalImaging => "Binary medical imaging",
            Self::LeafDiseaseClassification => "Leaf disease classification",
        }
    }
}

#[derive(Debug)]
pub enum WorkspaceError {
    Io(io::Error),
    Decode(toml::de::Error),
    Encode(toml::ser::Error),
}

impl From<io::Error> for WorkspaceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml::de::Error> for WorkspaceError {
    fn from(error: toml::de::Error) -> Self {
        Self::Decode(error)
    }
}

impl From<toml::ser::Error> for WorkspaceError {
    fn from(error: toml::ser::Error) -> Self {
        Self::Encode(error)
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "workspace I/O error: {error}"),
            Self::Decode(error) => write!(formatter, "workspace decode error: {error}"),
            Self::Encode(error) => write!(formatter, "workspace encode error: {error}"),
        }
    }
}

impl Error for WorkspaceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_loads_project_manifest() {
        let temp = tempfile::tempdir().expect("tempdir");
        let manifest = ProjectManifest::new(
            "Leaf disease study",
            ResearchTemplate::LeafDiseaseClassification,
        );

        let manifest_path = manifest.save(temp.path()).expect("save manifest");
        let loaded = ProjectManifest::load(temp.path()).expect("load manifest");

        assert_eq!(manifest_path, temp.path().join(PROJECT_FILE_NAME));
        assert_eq!(loaded, manifest);
        assert_eq!(
            loaded.project.template.label(),
            "Leaf disease classification"
        );
        assert_eq!(loaded.artifact_paths(temp.path()).len(), 6);
        assert!(temp.path().join("metadata").is_dir());
        assert!(temp.path().join("reports/predictions").is_dir());
        assert!(temp.path().join("reports/figures").is_dir());
        assert!(temp.path().join("manuscript").is_dir());
        assert!(temp.path().join(".vision-workbench/agents").is_dir());
    }
}
