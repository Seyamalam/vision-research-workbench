use crate::workspace::ResearchTemplate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSpec {
    pub id: CommandId,
    pub label: &'static str,
    pub description: &'static str,
    pub risk: CommandRisk,
    pub permission: PermissionGate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandId {
    CreateProject,
    OpenProject,
    ImportDataset,
    AuditDuplicates,
    GenerateSplits,
    ImportPredictions,
    EvaluateMetrics,
    ExportReport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandRisk {
    ReadOnly,
    WritesProject,
    Expensive,
    Destructive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionGate {
    None,
    ApprovalRequired,
    AgentApprovalRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CommandPlan {
    CreateProject {
        root: PathBuf,
        name: String,
        template: ResearchTemplate,
    },
    OpenProject {
        root: PathBuf,
    },
    ImportDataset {
        path: PathBuf,
    },
    AuditDuplicates,
    GenerateSplits {
        seed: u64,
    },
    ImportPredictions {
        path: String,
    },
    EvaluateMetrics,
    ExportReport,
}

pub fn registry() -> Vec<CommandSpec> {
    vec![
        CommandSpec {
            id: CommandId::CreateProject,
            label: "Create project",
            description: "Create a project manifest and default artifact folders.",
            risk: CommandRisk::WritesProject,
            permission: PermissionGate::None,
        },
        CommandSpec {
            id: CommandId::OpenProject,
            label: "Open project",
            description: "Load an existing project manifest and make it active.",
            risk: CommandRisk::ReadOnly,
            permission: PermissionGate::None,
        },
        CommandSpec {
            id: CommandId::ImportDataset,
            label: "Import dataset",
            description: "Index images from a folder or manifest without copying raw data.",
            risk: CommandRisk::WritesProject,
            permission: PermissionGate::ApprovalRequired,
        },
        CommandSpec {
            id: CommandId::AuditDuplicates,
            label: "Audit duplicates",
            description: "Compute exact and perceptual duplicate groups for imported images.",
            risk: CommandRisk::Expensive,
            permission: PermissionGate::AgentApprovalRequired,
        },
        CommandSpec {
            id: CommandId::GenerateSplits,
            label: "Generate splits",
            description: "Create leakage-aware train, validation, and test split manifests.",
            risk: CommandRisk::WritesProject,
            permission: PermissionGate::ApprovalRequired,
        },
        CommandSpec {
            id: CommandId::ImportPredictions,
            label: "Import predictions",
            description: "Load validation and test prediction CSV files for evaluation.",
            risk: CommandRisk::WritesProject,
            permission: PermissionGate::ApprovalRequired,
        },
        CommandSpec {
            id: CommandId::EvaluateMetrics,
            label: "Evaluate metrics",
            description: "Compute metrics, confidence intervals, calibration, and thresholds.",
            risk: CommandRisk::Expensive,
            permission: PermissionGate::AgentApprovalRequired,
        },
        CommandSpec {
            id: CommandId::ExportReport,
            label: "Export report",
            description: "Write tables, figures, and manuscript-ready report artifacts.",
            risk: CommandRisk::WritesProject,
            permission: PermissionGate::ApprovalRequired,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_unique_command_ids() {
        let registry = registry();
        let mut ids = registry
            .iter()
            .map(|command| command.id)
            .collect::<Vec<_>>();
        ids.sort_by_key(|id| format!("{id:?}"));
        ids.dedup();

        assert_eq!(ids.len(), registry.len());
        assert!(
            registry
                .iter()
                .any(|command| command.permission == PermissionGate::AgentApprovalRequired)
        );
    }
}
