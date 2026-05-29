# PCOS XAI Workbench

A native macOS desktop application concept for auditing, training, evaluating, and reporting PCOS ultrasound AI experiments.

The target stack is **100% Rust for the desktop app**, using **GPUI** for a GPU-accelerated Apple Silicon-friendly interface. The app should make the research workflow reproducible from dataset intake through manuscript-ready figures, tables, and reports.

## Why This App

The original research workflow included dataset parsing, duplicate detection, leakage-controlled splitting, supervised and self-supervised baselines, calibration, robustness, explainability, and manuscript generation. This project turns that workflow into a local desktop workbench for repeatable experimentation.

## Proposed Product Scope

- Import and audit the PCOS-XAI ultrasound dataset.
- Build image metadata and readability checks.
- Detect exact duplicates with MD5.
- Detect perceptual near-duplicates with pHash.
- Flag cross-label visual conflicts.
- Generate duplicate and conflict panels.
- Create leakage-controlled splits.
- Train and evaluate baseline classifiers.
- Run low-label and multi-seed comparisons.
- Tune thresholds on validation predictions only.
- Compute confidence intervals and bootstrap metrics.
- Evaluate calibration and robustness.
- Generate Grad-CAM/XAI audit panels.
- Export manuscript-ready figures, tables, and reproducibility artifacts.

## Architecture Direction

### Desktop Shell

- Rust 2021/2024 edition.
- GPUI for the native macOS UI.
- Apple Silicon-first builds on `aarch64-apple-darwin`.
- Local-first project workspace model.
- Async task runner for long jobs.
- Persistent job history and experiment registry.

### Data Layer

- `rusqlite` or `sqlx` with SQLite for local experiment state.
- `polars` or Arrow/Parquet for tabular metadata and report exports.
- Content-addressed file index using MD5 plus optional stronger hashes.
- Image metadata extraction through Rust image crates.

### ML/Compute Layer

- Keep orchestration in Rust.
- Prefer `candle` for Rust-native neural network inference/training prototypes.
- Use `tch-rs` only if PyTorch compatibility is essential.
- Use Core ML export/inference where Apple Silicon deployment performance matters.
- Treat large training jobs as resumable background tasks with structured logs.

### Reporting Layer

- CSV, Parquet, PNG, SVG/PDF export.
- Reproducible figure manifest.
- Manuscript table export.
- Experiment card export for each run.

## Repository Status

This repository currently contains the planning and implementation roadmap. The first implementation milestone should scaffold the Rust workspace and verify a minimal GPUI window on macOS.

## Recommended Repository Name

`pcos-xai-workbench`

## Initial Milestones

1. Scaffold Rust workspace and minimal GPUI app.
2. Implement project workspace creation/opening.
3. Implement dataset import and metadata indexing.
4. Add duplicate and near-duplicate audit views.
5. Add leakage-aware split generation.
6. Add prediction import and evaluation dashboard.
7. Add calibration, robustness, and XAI report modules.
8. Add manuscript export pipeline.

## Development Prerequisites

- macOS on Apple Silicon.
- Latest stable Rust toolchain.
- Xcode Command Line Tools.
- GitHub CLI for repository publishing.

## References

- GPUI docs: https://docs.rs/gpui
- GPUI site: https://www.gpui.rs/
- GPUI source in Zed: https://github.com/zed-industries/zed/tree/main/crates/gpui

