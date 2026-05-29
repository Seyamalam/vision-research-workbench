use crate::commands::{CommandPlan, CommandRisk, CommandSpec, PermissionGate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub command_label: String,
    pub reason: String,
    pub gate: PermissionGate,
    pub risk: CommandRisk,
    pub plan: CommandPlan,
}

pub fn approval_request_for(spec: &CommandSpec, plan: CommandPlan) -> Option<ApprovalRequest> {
    if spec.permission == PermissionGate::None {
        return None;
    }

    Some(ApprovalRequest {
        command_label: spec.label.to_string(),
        reason: approval_reason(spec),
        gate: spec.permission,
        risk: spec.risk,
        plan,
    })
}

fn approval_reason(spec: &CommandSpec) -> String {
    match (spec.permission, spec.risk) {
        (PermissionGate::ApprovalRequired, CommandRisk::WritesProject) => {
            "This command writes project artifacts.".to_string()
        }
        (PermissionGate::AgentApprovalRequired, CommandRisk::Expensive) => {
            "This command can be expensive and should be approved before an agent runs it."
                .to_string()
        }
        (PermissionGate::AgentApprovalRequired, _) => {
            "This command requires approval before an agent runs it.".to_string()
        }
        _ => "This command requires approval.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{CommandId, registry};

    #[test]
    fn creates_approval_request_for_gated_command() {
        let spec = registry()
            .into_iter()
            .find(|command| command.id == CommandId::AuditDuplicates)
            .expect("audit command");

        let request = approval_request_for(
            &spec,
            CommandPlan::AuditDuplicates {
                path: "dataset".into(),
            },
        )
        .expect("approval request");

        assert_eq!(request.gate, PermissionGate::AgentApprovalRequired);
        assert_eq!(request.risk, CommandRisk::Expensive);
    }

    #[test]
    fn skips_approval_for_ungated_command() {
        let spec = registry()
            .into_iter()
            .find(|command| command.id == CommandId::CreateProject)
            .expect("create command");

        let request = approval_request_for(
            &spec,
            CommandPlan::CreateProject {
                root: "project".into(),
                name: "Project".to_string(),
                template: crate::workspace::ResearchTemplate::GenericImageClassification,
            },
        );

        assert!(request.is_none());
    }
}
