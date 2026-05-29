use crate::{dataset::is_supported_image_path, workspace::ProjectManifest};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    ffi::OsStr,
    fmt, fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub const EXACT_DUPLICATES_CSV_NAME: &str = "exact_duplicates.csv";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactDuplicateRecord {
    pub group_id: usize,
    pub md5: String,
    pub path: PathBuf,
    pub class_name: Option<String>,
    pub file_size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactDuplicateSummary {
    pub duplicate_groups: usize,
    pub duplicate_files_beyond_first: usize,
    pub output_csv: PathBuf,
}

pub fn audit_exact_duplicates(
    dataset_root: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
    project: &ProjectManifest,
) -> Result<ExactDuplicateSummary, DuplicateError> {
    let records = find_exact_duplicates(dataset_root)?;
    let duplicate_groups = records
        .iter()
        .map(|record| record.group_id)
        .collect::<BTreeSet<_>>()
        .len();
    let duplicate_files_beyond_first = records.len().saturating_sub(duplicate_groups);
    let output_csv = project_root
        .as_ref()
        .join(&project.artifacts.metadata_dir)
        .join(EXACT_DUPLICATES_CSV_NAME);
    write_exact_duplicate_csv(&records, &output_csv)?;

    Ok(ExactDuplicateSummary {
        duplicate_groups,
        duplicate_files_beyond_first,
        output_csv,
    })
}

pub fn find_exact_duplicates(
    dataset_root: impl AsRef<Path>,
) -> Result<Vec<ExactDuplicateRecord>, DuplicateError> {
    let dataset_root = dataset_root.as_ref();
    let mut by_hash: BTreeMap<String, Vec<(PathBuf, Option<String>, u64)>> = BTreeMap::new();

    for entry in WalkDir::new(dataset_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if !is_supported_image_path(path) {
            continue;
        }

        let bytes = fs::read(path)?;
        let digest = format!("{:x}", Md5::digest(&bytes));
        let relative_path = path
            .strip_prefix(dataset_root)
            .unwrap_or(path)
            .to_path_buf();
        let class_name = relative_path
            .parent()
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .map(ToOwned::to_owned);
        by_hash
            .entry(digest)
            .or_default()
            .push((relative_path, class_name, bytes.len() as u64));
    }

    let mut records = Vec::new();
    for (group_id, (md5, files)) in by_hash
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .enumerate()
    {
        for (path, class_name, file_size) in files {
            records.push(ExactDuplicateRecord {
                group_id,
                md5: md5.clone(),
                path,
                class_name,
                file_size,
            });
        }
    }

    records.sort_by(|left, right| {
        left.group_id
            .cmp(&right.group_id)
            .then_with(|| left.path.cmp(&right.path))
    });
    Ok(records)
}

pub fn write_exact_duplicate_csv(
    records: &[ExactDuplicateRecord],
    output_path: impl AsRef<Path>,
) -> Result<(), DuplicateError> {
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

#[derive(Debug)]
pub enum DuplicateError {
    Io(io::Error),
    Csv(csv::Error),
}

impl From<io::Error> for DuplicateError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<csv::Error> for DuplicateError {
    fn from(error: csv::Error) -> Self {
        Self::Csv(error)
    }
}

impl fmt::Display for DuplicateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "duplicate audit I/O error: {error}"),
            Self::Csv(error) => write!(formatter, "duplicate audit CSV error: {error}"),
        }
    }
}

impl Error for DuplicateError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ProjectManifest, ResearchTemplate};

    #[test]
    fn finds_exact_duplicate_image_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let dataset_root = temp.path().join("dataset");
        let class_dir = dataset_root.join("class-a");
        fs::create_dir_all(&class_dir).expect("class dir");
        fs::write(class_dir.join("a.png"), b"same bytes").expect("a");
        fs::write(class_dir.join("b.png"), b"same bytes").expect("b");
        fs::write(class_dir.join("c.png"), b"different bytes").expect("c");

        let project_root = temp.path().join("project");
        let manifest = ProjectManifest::new("Test", ResearchTemplate::GenericImageClassification);
        manifest.save(&project_root).expect("project");

        let summary =
            audit_exact_duplicates(&dataset_root, &project_root, &manifest).expect("audit");

        assert_eq!(summary.duplicate_groups, 1);
        assert_eq!(summary.duplicate_files_beyond_first, 1);
        assert!(summary.output_csv.is_file());
    }
}
