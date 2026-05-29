use crate::workspace::{ProjectManifest, WorkspaceError};
use image::{GenericImageView, ImageReader};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    ffi::OsStr,
    fmt, fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub const IMAGES_CSV_NAME: &str = "images.csv";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageRecord {
    pub path: PathBuf,
    pub class_name: Option<String>,
    pub label: Option<String>,
    pub readable: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color_type: Option<String>,
    pub suffix: Option<String>,
    pub file_size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatasetImportSummary {
    pub total_files: usize,
    pub readable_images: usize,
    pub unreadable_images: usize,
    pub output_csv: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetSummary {
    pub total_files: usize,
    pub readable_images: usize,
    pub unreadable_images: usize,
    pub class_counts: BTreeMap<String, usize>,
}

pub fn import_image_folder(
    dataset_root: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
    project: &ProjectManifest,
) -> Result<DatasetImportSummary, DatasetError> {
    let dataset_root = dataset_root.as_ref();
    let project_root = project_root.as_ref();
    let records = scan_image_folder(dataset_root)?;
    let output_csv = project_root
        .join(&project.artifacts.metadata_dir)
        .join(IMAGES_CSV_NAME);
    write_image_records_csv(&records, &output_csv)?;

    Ok(DatasetImportSummary {
        total_files: records.len(),
        readable_images: records.iter().filter(|record| record.readable).count(),
        unreadable_images: records.iter().filter(|record| !record.readable).count(),
        output_csv,
    })
}

pub fn scan_image_folder(dataset_root: impl AsRef<Path>) -> Result<Vec<ImageRecord>, DatasetError> {
    let dataset_root = dataset_root.as_ref();
    let mut records = Vec::new();

    for entry in WalkDir::new(dataset_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if !is_supported_image_path(path) {
            continue;
        }

        records.push(read_image_record(dataset_root, path)?);
    }

    records.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(records)
}

pub fn write_image_records_csv(
    records: &[ImageRecord],
    output_path: impl AsRef<Path>,
) -> Result<(), DatasetError> {
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

pub fn summarize_records(records: &[ImageRecord]) -> DatasetSummary {
    let mut class_counts = BTreeMap::new();
    for record in records.iter().filter(|record| record.readable) {
        let class_name = record
            .class_name
            .clone()
            .unwrap_or_else(|| "unlabeled".to_string());
        *class_counts.entry(class_name).or_insert(0) += 1;
    }

    DatasetSummary {
        total_files: records.len(),
        readable_images: records.iter().filter(|record| record.readable).count(),
        unreadable_images: records.iter().filter(|record| !record.readable).count(),
        class_counts,
    }
}

fn read_image_record(dataset_root: &Path, path: &Path) -> Result<ImageRecord, DatasetError> {
    let metadata = fs::metadata(path)?;
    let relative_path = path
        .strip_prefix(dataset_root)
        .unwrap_or(path)
        .to_path_buf();
    let class_name = relative_path
        .parent()
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(ToOwned::to_owned);
    let suffix = path
        .extension()
        .and_then(OsStr::to_str)
        .map(|suffix| suffix.to_ascii_lowercase());

    let decoded = ImageReader::open(path)
        .and_then(|reader| reader.with_guessed_format())
        .ok()
        .and_then(|reader| reader.decode().ok());

    let (readable, width, height, color_type) = match decoded {
        Some(image) => {
            let (width, height) = image.dimensions();
            (
                true,
                Some(width),
                Some(height),
                Some(format!("{:?}", image.color())),
            )
        }
        None => (false, None, None, None),
    };

    Ok(ImageRecord {
        path: relative_path,
        label: class_name.clone(),
        class_name,
        readable,
        width,
        height,
        color_type,
        suffix,
        file_size: metadata.len(),
    })
}

pub fn is_supported_image_path(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(OsStr::to_str)
            .map(|extension| extension.to_ascii_lowercase())
            .as_deref(),
        Some("bmp" | "gif" | "jpeg" | "jpg" | "png" | "tif" | "tiff" | "webp")
    )
}

#[derive(Debug)]
pub enum DatasetError {
    Io(io::Error),
    Csv(csv::Error),
    Workspace(WorkspaceError),
}

impl From<io::Error> for DatasetError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<csv::Error> for DatasetError {
    fn from(error: csv::Error) -> Self {
        Self::Csv(error)
    }
}

impl From<WorkspaceError> for DatasetError {
    fn from(error: WorkspaceError) -> Self {
        Self::Workspace(error)
    }
}

impl fmt::Display for DatasetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "dataset I/O error: {error}"),
            Self::Csv(error) => write!(formatter, "dataset CSV error: {error}"),
            Self::Workspace(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for DatasetError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ProjectManifest, ResearchTemplate};
    use image::{ImageBuffer, Rgb};

    #[test]
    fn scans_images_and_writes_metadata_csv() {
        let temp = tempfile::tempdir().expect("tempdir");
        let dataset_root = temp.path().join("dataset");
        let healthy_dir = dataset_root.join("healthy");
        let disease_dir = dataset_root.join("disease");
        fs::create_dir_all(&healthy_dir).expect("healthy dir");
        fs::create_dir_all(&disease_dir).expect("disease dir");

        let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(8, 6, Rgb([12, 34, 56]));
        image
            .save(healthy_dir.join("sample.png"))
            .expect("save readable image");
        fs::write(disease_dir.join("broken.jpg"), b"not an image").expect("broken image");

        let project_root = temp.path().join("project");
        let manifest = ProjectManifest::new("Test", ResearchTemplate::GenericImageClassification);
        manifest.save(&project_root).expect("save project");

        let summary =
            import_image_folder(&dataset_root, &project_root, &manifest).expect("import dataset");

        assert_eq!(summary.total_files, 2);
        assert_eq!(summary.readable_images, 1);
        assert_eq!(summary.unreadable_images, 1);
        assert!(summary.output_csv.is_file());
    }

    #[test]
    fn summarizes_readable_images_by_class() {
        let records = vec![
            ImageRecord {
                path: "healthy/a.png".into(),
                class_name: Some("healthy".to_string()),
                label: Some("healthy".to_string()),
                readable: true,
                width: Some(10),
                height: Some(10),
                color_type: Some("Rgb8".to_string()),
                suffix: Some("png".to_string()),
                file_size: 10,
            },
            ImageRecord {
                path: "healthy/b.png".into(),
                class_name: Some("healthy".to_string()),
                label: Some("healthy".to_string()),
                readable: true,
                width: Some(10),
                height: Some(10),
                color_type: Some("Rgb8".to_string()),
                suffix: Some("png".to_string()),
                file_size: 10,
            },
            ImageRecord {
                path: "disease/broken.jpg".into(),
                class_name: Some("disease".to_string()),
                label: Some("disease".to_string()),
                readable: false,
                width: None,
                height: None,
                color_type: None,
                suffix: Some("jpg".to_string()),
                file_size: 10,
            },
        ];

        let summary = summarize_records(&records);

        assert_eq!(summary.total_files, 3);
        assert_eq!(summary.readable_images, 2);
        assert_eq!(summary.unreadable_images, 1);
        assert_eq!(summary.class_counts.get("healthy"), Some(&2));
        assert_eq!(summary.class_counts.get("disease"), None);
    }
}
