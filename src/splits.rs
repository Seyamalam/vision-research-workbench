use crate::{
    dataset::{DatasetError, ImageRecord, scan_image_folder},
    workspace::ProjectManifest,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt, fs, hash,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
};

pub const SPLITS_CSV_NAME: &str = "splits.csv";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitRecord {
    pub path: PathBuf,
    pub class_name: Option<String>,
    pub label: Option<String>,
    pub split: Split,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Split {
    Train,
    Validation,
    Test,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitSummary {
    pub train: usize,
    pub validation: usize,
    pub test: usize,
    pub output_csv: PathBuf,
}

pub fn generate_splits_from_folder(
    dataset_root: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
    project: &ProjectManifest,
    seed: u64,
) -> Result<SplitSummary, SplitError> {
    let records = scan_image_folder(dataset_root)?;
    let split_records = generate_splits(&records, seed);
    let output_csv = project_root
        .as_ref()
        .join(&project.artifacts.metadata_dir)
        .join(SPLITS_CSV_NAME);
    write_splits_csv(&split_records, &output_csv)?;

    Ok(SplitSummary {
        train: split_records
            .iter()
            .filter(|record| record.split == Split::Train)
            .count(),
        validation: split_records
            .iter()
            .filter(|record| record.split == Split::Validation)
            .count(),
        test: split_records
            .iter()
            .filter(|record| record.split == Split::Test)
            .count(),
        output_csv,
    })
}

pub fn generate_splits(records: &[ImageRecord], seed: u64) -> Vec<SplitRecord> {
    let mut split_records = records
        .iter()
        .filter(|record| record.readable)
        .map(|record| {
            let bucket = deterministic_bucket(&record.path, seed);
            let split = match bucket {
                0..=79 => Split::Train,
                80..=89 => Split::Validation,
                _ => Split::Test,
            };
            SplitRecord {
                path: record.path.clone(),
                class_name: record.class_name.clone(),
                label: record.label.clone(),
                split,
            }
        })
        .collect::<Vec<_>>();

    split_records.sort_by(|left, right| left.path.cmp(&right.path));
    split_records
}

pub fn write_splits_csv(
    records: &[SplitRecord],
    output_path: impl AsRef<Path>,
) -> Result<(), SplitError> {
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut writer = csv::Writer::from_path(output_path)?;
    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}

fn deterministic_bucket(path: &Path, seed: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    path.hash(&mut hasher);
    hasher.finish() % 100
}

#[derive(Debug)]
pub enum SplitError {
    Io(io::Error),
    Csv(csv::Error),
    Dataset(DatasetError),
}

impl From<io::Error> for SplitError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<csv::Error> for SplitError {
    fn from(error: csv::Error) -> Self {
        Self::Csv(error)
    }
}

impl From<DatasetError> for SplitError {
    fn from(error: DatasetError) -> Self {
        Self::Dataset(error)
    }
}

impl fmt::Display for SplitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "split I/O error: {error}"),
            Self::Csv(error) => write!(formatter, "split CSV error: {error}"),
            Self::Dataset(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for SplitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_generation_is_deterministic() {
        let records = (0..20)
            .map(|index| ImageRecord {
                path: format!("class/{index}.png").into(),
                class_name: Some("class".to_string()),
                label: Some("class".to_string()),
                readable: true,
                width: Some(1),
                height: Some(1),
                color_type: Some("Rgb8".to_string()),
                suffix: Some("png".to_string()),
                file_size: 1,
            })
            .collect::<Vec<_>>();

        let first = generate_splits(&records, 42);
        let second = generate_splits(&records, 42);

        assert_eq!(first, second);
        assert_eq!(first.len(), 20);
    }
}
