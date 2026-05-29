use crate::{dataset::is_supported_image_path, workspace::ProjectManifest};
use image::{DynamicImage, ImageReader, imageops::FilterType};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    ffi::OsStr,
    fmt, fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub const PHASH_DUPLICATES_CSV_NAME: &str = "phash_near_duplicates.csv";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerceptualHashRecord {
    pub group_id: usize,
    pub hash: u64,
    pub path: PathBuf,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerceptualDuplicateSummary {
    pub groups: usize,
    pub files_beyond_first: usize,
    pub cross_label_groups: usize,
    pub output_csv: PathBuf,
}

pub fn audit_perceptual_duplicates(
    dataset_root: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
    project: &ProjectManifest,
    max_hamming_distance: u32,
) -> Result<PerceptualDuplicateSummary, PerceptualError> {
    let records = find_perceptual_duplicates(&dataset_root, max_hamming_distance)?;
    let groups = records
        .iter()
        .map(|record| record.group_id)
        .collect::<BTreeSet<_>>()
        .len();
    let files_beyond_first = records.len().saturating_sub(groups);
    let cross_label_groups = count_cross_label_groups(&records);
    let output_csv = project_root
        .as_ref()
        .join(&project.artifacts.metadata_dir)
        .join(PHASH_DUPLICATES_CSV_NAME);
    write_perceptual_duplicate_csv(&records, &output_csv)?;

    Ok(PerceptualDuplicateSummary {
        groups,
        files_beyond_first,
        cross_label_groups,
        output_csv,
    })
}

pub fn find_perceptual_duplicates(
    dataset_root: impl AsRef<Path>,
    max_hamming_distance: u32,
) -> Result<Vec<PerceptualHashRecord>, PerceptualError> {
    let dataset_root = dataset_root.as_ref();
    let mut images = Vec::new();

    for entry in WalkDir::new(dataset_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if !is_supported_image_path(path) {
            continue;
        }

        let Some(hash) = perceptual_hash(path)? else {
            continue;
        };
        let relative_path = path
            .strip_prefix(dataset_root)
            .unwrap_or(path)
            .to_path_buf();
        let class_name = relative_path
            .parent()
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .map(ToOwned::to_owned);
        images.push((hash, relative_path, class_name));
    }

    let mut parent = (0..images.len()).collect::<Vec<_>>();
    for i in 0..images.len() {
        for j in (i + 1)..images.len() {
            if hamming_distance(images[i].0, images[j].0) <= max_hamming_distance {
                union(&mut parent, i, j);
            }
        }
    }

    let mut grouped: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for index in 0..images.len() {
        grouped
            .entry(find(&mut parent, index))
            .or_default()
            .push(index);
    }

    let mut records = Vec::new();
    for (group_id, indices) in grouped
        .into_values()
        .filter(|indices| indices.len() > 1)
        .enumerate()
    {
        for index in indices {
            let (hash, path, class_name) = images[index].clone();
            records.push(PerceptualHashRecord {
                group_id,
                hash,
                path,
                class_name,
            });
        }
    }

    Ok(records)
}

pub fn perceptual_hash(path: &Path) -> Result<Option<u64>, PerceptualError> {
    let decoded = ImageReader::open(path)
        .and_then(|reader| reader.with_guessed_format())
        .ok()
        .and_then(|reader| reader.decode().ok());

    Ok(decoded.map(average_hash))
}

fn average_hash(image: DynamicImage) -> u64 {
    let gray = image.resize_exact(8, 8, FilterType::Triangle).to_luma8();
    let average = gray.pixels().map(|pixel| pixel[0] as u32).sum::<u32>() / 64;
    gray.pixels()
        .enumerate()
        .fold(0u64, |hash, (index, pixel)| {
            if pixel[0] as u32 >= average {
                hash | (1u64 << index)
            } else {
                hash
            }
        })
}

fn hamming_distance(left: u64, right: u64) -> u32 {
    (left ^ right).count_ones()
}

fn find(parent: &mut [usize], index: usize) -> usize {
    if parent[index] != index {
        parent[index] = find(parent, parent[index]);
    }
    parent[index]
}

fn union(parent: &mut [usize], left: usize, right: usize) {
    let left_root = find(parent, left);
    let right_root = find(parent, right);
    if left_root != right_root {
        parent[right_root] = left_root;
    }
}

fn count_cross_label_groups(records: &[PerceptualHashRecord]) -> usize {
    let mut labels_by_group: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
    for record in records {
        if let Some(class_name) = &record.class_name {
            labels_by_group
                .entry(record.group_id)
                .or_default()
                .insert(class_name.clone());
        }
    }
    labels_by_group
        .values()
        .filter(|labels| labels.len() > 1)
        .count()
}

fn write_perceptual_duplicate_csv(
    records: &[PerceptualHashRecord],
    output_path: impl AsRef<Path>,
) -> Result<(), PerceptualError> {
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
pub enum PerceptualError {
    Io(io::Error),
    Csv(csv::Error),
}

impl From<io::Error> for PerceptualError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<csv::Error> for PerceptualError {
    fn from(error: csv::Error) -> Self {
        Self::Csv(error)
    }
}

impl fmt::Display for PerceptualError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "perceptual audit I/O error: {error}"),
            Self::Csv(error) => write!(formatter, "perceptual audit CSV error: {error}"),
        }
    }
}

impl Error for PerceptualError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{ProjectManifest, ResearchTemplate};
    use image::{ImageBuffer, Rgb};

    #[test]
    fn finds_near_duplicate_images() {
        let temp = tempfile::tempdir().expect("tempdir");
        let dataset_root = temp.path().join("dataset");
        let class_a = dataset_root.join("a");
        let class_b = dataset_root.join("b");
        fs::create_dir_all(&class_a).expect("class a");
        fs::create_dir_all(&class_b).expect("class b");

        let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(16, 16, Rgb([10, 20, 30]));
        image.save(class_a.join("one.png")).expect("one");
        image.save(class_b.join("two.png")).expect("two");

        let project_root = temp.path().join("project");
        let manifest = ProjectManifest::new("Test", ResearchTemplate::GenericImageClassification);
        manifest.save(&project_root).expect("project");

        let summary =
            audit_perceptual_duplicates(&dataset_root, &project_root, &manifest, 0).expect("audit");

        assert_eq!(summary.groups, 1);
        assert_eq!(summary.files_beyond_first, 1);
        assert_eq!(summary.cross_label_groups, 1);
        assert!(summary.output_csv.is_file());
    }
}
