# TODO

This roadmap converts repeatable image AI research workflows into a native Rust/GPUI desktop application. PCOS ultrasound is the motivating example, but the product should support leaf disease detection, microscopy, industrial defects, satellite imagery, and other image-based research projects.

## 0. Repository And Project Setup

- [x] Initialize Rust workspace.
- [x] Add minimal GPUI macOS window.
- [ ] Add app icon and bundle metadata.
- [ ] Add `justfile` or `cargo-make` task runner.
- [ ] Add formatting, linting, and test commands.
- [ ] Add CI for `cargo fmt`, `cargo clippy`, and tests.
- [ ] Add release profile tuned for Apple Silicon.
- [ ] Add crash-safe local app data directory handling.

## 1. Product Foundation

- [x] Define workspace/project file format.
- [ ] Define dataset-agnostic project templates.
- [ ] Add template for binary medical imaging.
- [ ] Add template for plant/leaf disease classification.
- [x] Add template for generic multiclass image classification.
- [x] Add project manifest save/load support.
- [x] Add default artifact directory layout.
- [x] Add project creation command core.
- [x] Add project open command core.
- [ ] Implement create/open/recent project flows.
- [x] Implement persistent settings.
- [ ] Implement background job queue.
- [ ] Implement cancellable long-running jobs.
- [ ] Implement structured logs per job.
- [ ] Implement progress reporting for dataset, training, and export jobs.
- [ ] Implement error surfaces with recoverable actions.

## 2. Data Setup

- [x] Import image dataset root.
- [x] Add folder-based image dataset scan core.
- [x] Export image metadata to `metadata/images.csv`.
- [ ] Import image dataset from CSV manifest.
- [x] Parse folder structures into labels.
- [ ] Support binary label mapping.
- [ ] Support multiclass label mapping.
- [ ] Add PCOS-XAI template mapping `infected` to PCOS-positive.
- [ ] Add PCOS-XAI template mapping `noninfected` to healthy/non-PCOS.
- [ ] Add leaf disease template for healthy/disease or disease-class labels.
- [x] Build image metadata for all files.
- [x] Check image readability.
- [x] Record image width and height.
- [x] Record image mode/color type.
- [x] Record file suffix.
- [x] Record file size.
- [x] Record class name.
- [x] Record labels from class folders.
- [x] Export `metadata/images.csv`.
- [ ] Add dataset overview screen.
- [ ] Add visible dataset import UI.
- [ ] Add unreadable image report.

## 3. Dataset Audit

- [x] Count total readable images.
- [ ] Count class balance.
- [x] Compute exact file hashes using MD5.
- [x] Find exact duplicate groups.
- [x] Count exact duplicate groups.
- [x] Count duplicate files beyond first copy.
- [x] Export exact duplicate metadata.
- [ ] Compute perceptual hashes with pHash.
- [ ] Group pHash near-duplicates by configurable Hamming radius.
- [ ] Count pHash near-duplicate groups.
- [ ] Count pHash near-duplicate files beyond first representative.
- [ ] Detect cross-label pHash near-duplicate groups.
- [ ] Count cross-label pHash groups.
- [ ] Generate duplicate examples.
- [ ] Generate cross-label duplicate examples.
- [ ] Build visual duplicate/conflict panel.
- [ ] Add interactive duplicate browser.
- [ ] Add visible exact duplicate audit UI.
- [ ] Add cross-label conflict triage workflow.

## 4. Leakage-Controlled Splitting

- [ ] Create duplicate-aware train/validation/test splits.
- [ ] Create stricter pHash near-duplicate-aware splits.
- [ ] Prevent near-duplicate groups from crossing train/validation/test.
- [ ] Exclude cross-label pHash groups from strict evaluation.
- [ ] Export `metadata/splits_near_duplicate_aware_phash.csv` or project-specific split path.
- [ ] Display final strict split counts.
- [ ] Add split reproducibility seed control.
- [ ] Add split manifest export.

## 5. Model Baselines

- [ ] Add supervised ImageNet-transfer baseline runner.
- [ ] Add ResNet-18 baseline.
- [ ] Add EfficientNet-B0 baseline.
- [ ] Add ViT-Tiny baseline.
- [ ] Add ConvNeXt-Tiny baseline.
- [ ] Report accuracy.
- [ ] Report AUROC.
- [ ] Report AUPRC.
- [ ] Report F1.
- [ ] Report expected calibration error.
- [ ] Report sensitivity.
- [ ] Report specificity.
- [ ] Report confusion counts.
- [ ] Store model config, seed, split, and artifact paths.

## 6. Self-Supervised Learning

- [ ] Implement SimCLR experiment type.
- [ ] Implement BYOL experiment type.
- [ ] Use ResNet-18 encoders for matched SSL comparison.
- [ ] Pretrain SSL encoders using unlabeled training images.
- [ ] Fine-tune downstream classifiers with limited labels.
- [ ] Compare supervised transfer against SimCLR and BYOL.
- [ ] Store pretraining logs and checkpoints.
- [ ] Store fine-tuning logs and checkpoints.

## 7. Low-Label Experiments

- [ ] Run 10% label-budget experiments.
- [ ] Run 50% label-budget experiments.
- [ ] Run three-seed experiments with seeds 42, 7, and 123.
- [ ] Compare default-threshold accuracy.
- [ ] Compare validation-selected threshold accuracy.
- [ ] Summarize multi-seed means.
- [ ] Summarize multi-seed standard deviations.
- [ ] Export `reports/seed_summary.csv`.
- [ ] Add label-efficiency dashboard.

## 8. Threshold And Operating-Point Analysis

- [ ] Export validation predictions.
- [ ] Export test predictions.
- [ ] Select thresholds on validation predictions only.
- [ ] Lock thresholds before test evaluation.
- [ ] Compute tuned accuracy.
- [ ] Compute tuned sensitivity.
- [ ] Compute tuned specificity.
- [ ] Compute Wilson confidence interval for accuracy.
- [ ] Compute Wilson confidence interval for sensitivity.
- [ ] Compute Wilson confidence interval for specificity.
- [ ] Create threshold confidence interval tables.
- [ ] Create threshold confidence interval plots.
- [ ] Add threshold selection audit trail.

## 9. Uncertainty Analysis

- [ ] Bootstrap AUROC.
- [ ] Bootstrap AUPRC.
- [ ] Compute AUROC confidence intervals.
- [ ] Compute AUPRC confidence intervals.
- [ ] Detect and report saturated ranking metrics.
- [ ] Add bootstrap AUROC figure.
- [ ] Add bootstrap AUPRC figure.
- [ ] Add configurable bootstrap iteration count.

## 10. Calibration

- [ ] Measure Brier score.
- [ ] Measure expected calibration error.
- [ ] Measure negative log-likelihood.
- [ ] Apply Platt scaling.
- [ ] Apply temperature scaling.
- [ ] Compare uncalibrated predictions against calibrated predictions.
- [ ] Create calibration curves.
- [ ] Create calibration improvement figures.
- [ ] Export calibration summary tables.
- [ ] Add calibration dashboard.

## 11. Robustness Testing

- [ ] Build corruption/severity evaluation module.
- [ ] Test blur robustness.
- [ ] Test downsampling robustness.
- [ ] Test contrast changes.
- [ ] Test center-crop/zoom perturbations.
- [ ] Test Gaussian noise.
- [ ] Compute clean accuracy.
- [ ] Compute mean corruption accuracy.
- [ ] Compute worst corruption accuracy.
- [ ] Compute mean degradation.
- [ ] Compute worst degradation.
- [ ] Build robustness severity summary.
- [ ] Build robustness condition heatmap.
- [ ] Show clean accuracy versus robustness degradation.
- [ ] Add configurable corruption severity levels.

## 12. Explainability / XAI

- [ ] Generate Grad-CAM explanations.
- [ ] Compare supervised ResNet-18 Grad-CAM outputs.
- [ ] Compare SimCLR Grad-CAM outputs.
- [ ] Generate negative/healthy example Grad-CAM panels.
- [ ] Generate positive/disease example Grad-CAM panels.
- [ ] Generate multiclass Grad-CAM panels.
- [ ] Run randomized-weight Grad-CAM sanity checks.
- [ ] Compare trained versus randomized Grad-CAM similarity.
- [ ] Measure trained-vs-random CAM Pearson correlation.
- [ ] Measure top-10 CAM overlap.
- [ ] Run border-mask perturbation tests.
- [ ] Measure prediction probability changes after border masking.
- [ ] Measure CAM border fraction.
- [ ] Export XAI sanity summary CSV.
- [ ] Create Grad-CAM comparison panel.
- [ ] Create Grad-CAM sanity/perturbation panel.
- [ ] Add XAI caution text: Grad-CAM is an audit, not proof of anatomical reasoning.

## 13. Manuscript Figures

- [ ] Build seed-summary accuracy figure.
- [ ] Build label-efficiency tuned-vs-default figure.
- [ ] Build threshold confidence interval figure.
- [ ] Build bootstrap AUROC confidence interval figure.
- [ ] Build calibration curve figure.
- [ ] Build calibration Brier improvement figure.
- [ ] Build robustness severity figure.
- [ ] Build robustness heatmap.
- [ ] Build dataset sample image panel.
- [ ] Build duplicate/conflict image panel.
- [ ] Build Grad-CAM comparison panel.
- [ ] Build Grad-CAM sanity panel.
- [ ] Build methodology workflow figure.
- [ ] Update figure manifest.
- [ ] Add figure preview browser.
- [ ] Add export presets for manuscript and supplementary material.

## 14. Manuscript Writing Support

- [ ] Store abstract draft.
- [ ] Store introduction draft.
- [ ] Store contributions.
- [ ] Store related work notes.
- [ ] Store domain literature notes.
- [ ] Store PCOS ultrasound AI literature notes as an optional template.
- [ ] Store plant disease detection literature notes as an optional template.
- [ ] Store SSL/medical imaging literature notes.
- [ ] Store reporting guideline references.
- [ ] Store dataset section.
- [ ] Store duplicate and near-duplicate control section.
- [ ] Store model section.
- [ ] Store training protocol section.
- [ ] Store threshold and confidence interval section.
- [ ] Store calibration/robustness/XAI section.
- [ ] Store results.
- [ ] Store discussion.
- [ ] Store limitations.
- [ ] Store conclusion.
- [ ] Store supplementary/reproducibility appendix notes.
- [ ] Add manuscript artifact checklist.

## 15. References

- [ ] Add foundational model references.
- [ ] Add ResNet reference.
- [ ] Add EfficientNet reference.
- [ ] Add ViT reference.
- [ ] Add ConvNeXt reference.
- [ ] Add SimCLR reference.
- [ ] Add BYOL reference.
- [ ] Add Grad-CAM reference.
- [ ] Add calibration references.
- [ ] Add recent PCOS ultrasound papers.
- [ ] Add recent SSL ultrasound/medical imaging papers.
- [ ] Add reporting-guideline papers.
- [ ] Export `manuscript/paper/references.bib`.
- [ ] Add reference validation.

## 16. Author And Submission Metadata

- [ ] Add four-author metadata support.
- [ ] Mark Touhidul Alam Seyam as corresponding author.
- [ ] Add BGC Trust University Bangladesh affiliation.
- [ ] Add author contribution statement editor.
- [ ] Rebuild manuscript PDF.
- [ ] Add submission metadata export.

## 17. Reproducibility

- [ ] Create result collection script equivalent.
- [ ] Create threshold evaluation script equivalent.
- [ ] Create calibration evaluation script equivalent.
- [ ] Create robustness evaluation script equivalent.
- [ ] Create XAI/Grad-CAM generation script equivalent.
- [ ] Create final figure generation script equivalent.
- [ ] Create manuscript image panel generation script equivalent.
- [ ] Preserve report CSVs.
- [ ] Preserve metadata CSVs.
- [ ] Preserve prediction exports.
- [ ] Preserve generated manuscript tables.
- [ ] Add full run manifest.
- [ ] Add environment manifest.
- [ ] Add deterministic seed registry.

## 18. Final Output Targets

- [ ] Export main LaTeX manuscript at `manuscript/paper/main.tex`.
- [ ] Export PDF manuscript at `manuscript/paper/main.pdf`.
- [ ] Export bibliography at `manuscript/paper/references.bib`.
- [ ] Export metadata audit files in `metadata/`.
- [ ] Export result summaries in `reports/`.
- [ ] Export prediction files in `reports/predictions/`.
- [ ] Export manuscript tables in `reports/manuscript_tables/`.
- [ ] Export manuscript figures in `reports/figures/`.
- [ ] Add one-click project archive export.

## 19. Agent And ACP Readiness

- [x] Design typed command registry shared by UI and agents.
- [ ] Define read-only project state API.
- [x] Define mutating command API with permission gates.
- [x] Add job-plan schema for agent-proposed work.
- [x] Add audit log for agent actions.
- [ ] Add artifact provenance for agent-created files.
- [ ] Add ACP-compatible transport spike.
- [ ] Add local agent connection manager.
- [ ] Add agent permission profiles.
- [x] Add approval request model for expensive jobs.
- [ ] Add manual approval UI for expensive jobs.
- [ ] Add manual approval UI for dataset label edits.
- [ ] Add manual approval UI for destructive artifact operations.
- [x] Add command dry-run support.
- [ ] Add machine-readable experiment summaries.
- [ ] Add machine-readable figure manifests.
- [ ] Add machine-readable manuscript/checklist state.
- [ ] Add Codex-oriented integration notes.

## 20. App UX

- [ ] Add sidebar navigation for Dataset, Audit, Splits, Models, Metrics, Calibration, Robustness, XAI, Manuscript, and Exports.
- [ ] Add dense table views for metadata and results.
- [ ] Add image grid with duplicate grouping.
- [ ] Add conflict review screen.
- [ ] Add experiment comparison dashboard.
- [ ] Add figure gallery.
- [ ] Add job monitor.
- [ ] Add settings screen.
- [ ] Add keyboard shortcuts.
- [ ] Add drag-and-drop dataset import.
- [ ] Add dark and light theme support.
- [ ] Add agent activity panel.
- [ ] Add command palette.
- [ ] Add project search.

## 21. Technical Risks To Resolve Early

- [ ] Verify GPUI packaging story for distributable `.app`.
- [x] Verify GPUI requires Apple's Metal Toolchain for shader compilation.
- [ ] Verify image table virtualization performance with 10k+ rows.
- [ ] Verify image grid performance with thousands of thumbnails.
- [ ] Decide Rust-native training versus external training bridge.
- [ ] Decide whether Core ML should be an inference-only backend.
- [ ] Validate Grad-CAM feasibility in the selected Rust ML stack.
- [ ] Validate reproducibility guarantees across Apple Silicon machines.
- [ ] Validate memory usage on large robustness and bootstrap jobs.
- [ ] Validate ACP integration maturity and expected protocol surface.
- [ ] Validate sandboxing and permission model for local agents.
