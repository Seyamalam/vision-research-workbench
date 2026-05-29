# Vision Research Workbench

A native macOS desktop application for image-based AI research workflows: dataset auditing, duplicate control, leakage-safe splitting, model evaluation, calibration, robustness testing, explainability, and manuscript-ready reporting.

The target stack is **100% Rust for the desktop app**, using **GPUI** for a GPU-accelerated Apple Silicon-friendly interface. The app should work for PCOS ultrasound research, leaf disease detection, pathology slides, industrial inspection, microscopy, satellite imagery, and other image-classification or image-audit projects.

## Why This App

Image AI research often repeats the same fragile workflow: organize folders, infer labels, audit duplicates, prevent leakage, train baselines, tune thresholds, quantify uncertainty, test robustness, generate explainability panels, and assemble manuscript artifacts. This project turns that workflow into a local-first desktop workbench.

The original motivating case was a PCOS ultrasound XAI paper, but the app should be dataset-agnostic. PCOS becomes one example template, not the product boundary.

## Product Scope

- Import image datasets from folder structures, CSV manifests, or project templates.
- Map arbitrary class names into binary or multiclass labels.
- Build image metadata and readability checks.
- Detect exact duplicates with file hashes.
- Detect perceptual near-duplicates with pHash or future embedding-based similarity.
- Flag cross-label visual conflicts.
- Generate duplicate, conflict, and sample panels.
- Create leakage-controlled train/validation/test splits.
- Train or import supervised and self-supervised model runs.
- Evaluate accuracy, AUROC, AUPRC, F1, calibration, sensitivity/specificity, and confusion counts.
- Run low-label and multi-seed comparisons.
- Tune thresholds on validation predictions only.
- Compute confidence intervals and bootstrap metrics.
- Evaluate calibration and robustness.
- Generate Grad-CAM/XAI audit panels where supported.
- Export manuscript-ready figures, tables, reports, and reproducibility artifacts.

## Example Research Templates

- Medical ultrasound binary classification, such as PCOS-positive versus healthy.
- Leaf disease detection from plant images.
- Crop stress or pest classification.
- Histopathology or microscopy image classification.
- Industrial defect detection.
- Satellite or drone image classification.
- General binary or multiclass image classification benchmarks.

## Architecture Direction

### Desktop Shell

- Rust 2021/2024 edition.
- GPUI for the native macOS UI.
- Apple Silicon-first builds on `aarch64-apple-darwin`.
- Local-first project workspace model.
- Async task runner for long jobs.
- Persistent job history and experiment registry.
- Extensible panels for dataset audit, experiments, metrics, XAI, manuscripts, and agents.

### Data Layer

- `rusqlite` or `sqlx` with SQLite for local experiment state.
- `polars` or Arrow/Parquet for tabular metadata and report exports.
- Content-addressed file index using MD5 plus optional stronger hashes.
- Image metadata extraction through Rust image crates.
- Dataset template definitions for common folder and manifest layouts.

### ML/Compute Layer

- Keep orchestration in Rust.
- Prefer `candle` for Rust-native neural network inference/training prototypes.
- Use `tch-rs` only if PyTorch compatibility is essential.
- Use Core ML export/inference where Apple Silicon deployment performance matters.
- Allow external command runners for heavyweight training while preserving Rust-owned provenance, logs, and artifacts.
- Treat large training jobs as resumable background tasks with structured logs.

### Agent And ACP Readiness

The app should be designed so future AI coding/research agents can safely inspect projects, propose experiments, run jobs, and summarize results.

- Add an internal command/action registry with typed inputs and outputs.
- Expose project state through stable read-only APIs.
- Keep every mutating operation auditable and reversible where practical.
- Store job plans, tool calls, logs, artifacts, and decisions in the project database.
- Design for Agent Client Protocol (ACP)-style integration so agents like Codex can connect to the workbench in a Zed-like workflow.
- Support permission gates before agents run expensive jobs, delete artifacts, export manuscripts, or change dataset labels.
- Provide machine-readable experiment manifests, result summaries, and figure manifests.
- Keep UI actions and agent actions backed by the same command layer.

### Reporting Layer

- CSV, Parquet, PNG, SVG/PDF export.
- Reproducible figure manifest.
- Manuscript table export.
- Experiment card export for each run.
- Project archive export for reviewers or collaborators.

## Repository Status

This repository now contains the first GPUI shell plus a reusable application core:

- A native Rust macOS window with a minimal sidebar and dashboard layout.
- A TOML project manifest named `vision-workbench.toml`.
- Default artifact folders for metadata, reports, predictions, figures, manuscripts, and agents.
- A typed command registry with risk levels and approval gates.
- A dry-run/apply execution path for project creation commands.

The next implementation step is to add project open/recent-project flows and persistent SQLite-backed state.

## Run Locally

```bash
cargo run
```

If GPUI fails while compiling Metal shaders, install Apple's Metal toolchain:

```bash
xcodebuild -downloadComponent MetalToolchain
```

## Repository Name

`vision-research-workbench`

## Initial Milestones

1. Scaffold Rust workspace and minimal GPUI app. Done.
2. Implement project workspace creation/opening. Creation core started.
3. Implement dataset import, label mapping, and metadata indexing.
4. Add duplicate and near-duplicate audit views.
5. Add leakage-aware split generation.
6. Add prediction import and evaluation dashboard.
7. Add calibration, robustness, and XAI report modules.
8. Add agent-ready command registry and project-state API. Command registry and dry-run started.
9. Add manuscript and reproducibility export pipeline.

## Development Prerequisites

- macOS on Apple Silicon.
- Latest stable Rust toolchain.
- Xcode Command Line Tools.
- GitHub CLI for repository publishing.

## References

- GPUI docs: https://docs.rs/gpui
- GPUI site: https://www.gpui.rs/
- GPUI source in Zed: https://github.com/zed-industries/zed/tree/main/crates/gpui
- Agent Client Protocol: https://agentclientprotocol.com/
