# TODO

This roadmap converts the completed PCOS ultrasound research workflow into a native Rust/GPUI desktop application.

## 0. Repository And Project Setup

- [ ] Initialize Rust workspace.
- [ ] Add minimal GPUI macOS window.
- [ ] Add app icon and bundle metadata.
- [ ] Add `justfile` or `cargo-make` task runner.
- [ ] Add formatting, linting, and test commands.
- [ ] Add CI for `cargo fmt`, `cargo clippy`, and tests.
- [ ] Add release profile tuned for Apple Silicon.
- [ ] Add crash-safe local app data directory handling.

## 1. Product Foundation

- [ ] Define workspace/project file format.
- [ ] Implement create/open/recent project flows.
- [ ] Implement persistent settings.
- [ ] Implement background job queue.
- [ ] Implement cancellable long-running jobs.
- [ ] Implement structured logs per job.
- [ ] Implement progress reporting for dataset, training, and export jobs.
- [ ] Implement error surfaces with recoverable actions.

## 2. Data Setup

- [ ] Import PCOS-XAI ultrasound dataset root.
- [ ] Parse dataset folder structure into binary labels.
- [ ] Map `infected` to PCOS-positive.
- [ ] Map `noninfected` to healthy/non-PCOS.
- [ ] Build image metadata for all files.
- [ ] Check image readability.
- [ ] Record image width and height.
- [ ] Record image mode/color type.
- [ ] Record file suffix.
- [ ] Record file size.
- [ ] Record class name.
- [ ] Record binary label.
- [ ] Export `metadata/images.csv`.
- [ ] Add dataset overview screen.
- [ ] Add unreadable image report.

## 3. Dataset Audit

- [ ] Count total readable images.
- [ ] Count class balance.
- [ ] Compute exact file hashes using MD5.
- [ ] Find exact duplicate groups.
- [ ] Count exact duplicate groups.
- [ ] Count duplicate files beyond first copy.
- [ ] Export exact duplicate metadata.
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
- [ ] Add cross-label conflict triage workflow.

## 4. Leakage-Controlled Splitting

- [ ] Create duplicate-aware train/validation/test splits.
- [ ] Create stricter pHash near-duplicate-aware splits.
- [ ] Prevent near-duplicate groups from crossing train/validation/test.
- [ ] Exclude cross-label pHash groups from strict evaluation.
- [ ] Export `metadata/splits_near_duplicate_aware_phash.csv`.
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
- [ ] Generate healthy example Grad-CAM panels.
- [ ] Generate PCOS-positive example Grad-CAM panels.
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
- [ ] Store PCOS ultrasound AI literature notes.
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

## 19. App UX

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

## 20. Technical Risks To Resolve Early

- [ ] Verify GPUI packaging story for distributable `.app`.
- [ ] Verify image table virtualization performance with 10k+ rows.
- [ ] Verify image grid performance with thousands of thumbnails.
- [ ] Decide Rust-native training versus external training bridge.
- [ ] Decide whether Core ML should be an inference-only backend.
- [ ] Validate Grad-CAM feasibility in the selected Rust ML stack.
- [ ] Validate reproducibility guarantees across Apple Silicon machines.
- [ ] Validate memory usage on large robustness and bootstrap jobs.

