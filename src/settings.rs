use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

pub const SETTINGS_FILE_NAME: &str = "settings.toml";
const MAX_RECENT_PROJECTS: usize = 10;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettings {
    pub last_project: Option<PathBuf>,
    pub recent_projects: Vec<PathBuf>,
}

impl AppSettings {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, SettingsError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }

        let encoded = fs::read_to_string(path)?;
        Ok(toml::from_str(&encoded)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), SettingsError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let encoded = toml::to_string_pretty(self)?;
        fs::write(path, encoded)?;
        Ok(())
    }

    pub fn remember_project(&mut self, root: impl Into<PathBuf>) {
        let root = root.into();
        self.last_project = Some(root.clone());
        self.recent_projects.retain(|project| project != &root);
        self.recent_projects.insert(0, root);
        self.recent_projects.truncate(MAX_RECENT_PROJECTS);
    }
}

#[derive(Debug)]
pub enum SettingsError {
    Io(io::Error),
    Decode(toml::de::Error),
    Encode(toml::ser::Error),
}

impl From<io::Error> for SettingsError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml::de::Error> for SettingsError {
    fn from(error: toml::de::Error) -> Self {
        Self::Decode(error)
    }
}

impl From<toml::ser::Error> for SettingsError {
    fn from(error: toml::ser::Error) -> Self {
        Self::Encode(error)
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "settings I/O error: {error}"),
            Self::Decode(error) => write!(formatter, "settings decode error: {error}"),
            Self::Encode(error) => write!(formatter, "settings encode error: {error}"),
        }
    }
}

impl Error for SettingsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembers_recent_projects_without_duplicates() {
        let mut settings = AppSettings::default();

        settings.remember_project("/tmp/a");
        settings.remember_project("/tmp/b");
        settings.remember_project("/tmp/a");

        assert_eq!(settings.last_project, Some(PathBuf::from("/tmp/a")));
        assert_eq!(
            settings.recent_projects,
            vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")]
        );
    }

    #[test]
    fn saves_and_loads_settings() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join(SETTINGS_FILE_NAME);
        let mut settings = AppSettings::default();
        settings.remember_project(temp.path().join("project"));

        settings.save(&path).expect("save settings");
        let loaded = AppSettings::load(&path).expect("load settings");

        assert_eq!(loaded, settings);
    }
}
