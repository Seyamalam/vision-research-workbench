use crate::{
    app_state::{CommandOutcome, ExecutionMode},
    commands::CommandPlan,
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const DATABASE_FILE_NAME: &str = "vision-workbench.sqlite";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAuditEvent {
    pub actor: String,
    pub mode: ExecutionMode,
    pub plan: CommandPlan,
    pub outcome: CommandOutcome,
    pub created_at_unix_ms: u128,
}

pub struct ProjectDatabase {
    connection: Connection,
}

impl ProjectDatabase {
    pub fn open(project_root: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let path = project_root.as_ref().join(DATABASE_FILE_NAME);
        let connection = Connection::open(path)?;
        let database = Self { connection };
        database.migrate()?;
        Ok(database)
    }

    pub fn migrate(&self) -> Result<(), DatabaseError> {
        self.connection.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS command_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                actor TEXT NOT NULL,
                mode TEXT NOT NULL,
                plan_json TEXT NOT NULL,
                outcome_json TEXT NOT NULL,
                created_at_unix_ms INTEGER NOT NULL
            );
            "#,
        )?;
        Ok(())
    }

    pub fn insert_command_event(&self, event: &CommandAuditEvent) -> Result<(), DatabaseError> {
        self.connection.execute(
            r#"
            INSERT INTO command_audit_log
                (actor, mode, plan_json, outcome_json, created_at_unix_ms)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                event.actor,
                serde_json::to_string(&event.mode)?,
                serde_json::to_string(&event.plan)?,
                serde_json::to_string(&event.outcome)?,
                event.created_at_unix_ms.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn command_event_count(&self) -> Result<u64, DatabaseError> {
        Ok(self
            .connection
            .query_row("SELECT COUNT(*) FROM command_audit_log", [], |row| {
                row.get::<_, u64>(0)
            })?)
    }
}

impl CommandAuditEvent {
    pub fn new(
        actor: impl Into<String>,
        mode: ExecutionMode,
        plan: CommandPlan,
        outcome: CommandOutcome,
    ) -> Self {
        Self {
            actor: actor.into(),
            mode,
            plan,
            outcome,
            created_at_unix_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis())
                .unwrap_or_default(),
        }
    }
}

pub fn database_path(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join(DATABASE_FILE_NAME)
}

#[derive(Debug)]
pub enum DatabaseError {
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "database error: {error}"),
            Self::Json(error) => write!(formatter, "database JSON error: {error}"),
        }
    }
}

impl Error for DatabaseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app_state::CommandOutcome,
        commands::CommandPlan,
        workspace::{ProjectManifest, ResearchTemplate},
    };

    #[test]
    fn stores_command_audit_events() {
        let temp = tempfile::tempdir().expect("tempdir");
        let project_root = temp.path().join("project");
        ProjectManifest::new("Project", ResearchTemplate::GenericImageClassification)
            .save(&project_root)
            .expect("project");
        let database = ProjectDatabase::open(&project_root).expect("database");

        let plan = CommandPlan::OpenProject {
            root: project_root.clone(),
        };
        let outcome = CommandOutcome {
            applied: true,
            summary: "Opened".to_string(),
            artifacts: vec![project_root.join("vision-workbench.toml")],
        };
        let event = CommandAuditEvent::new("test", ExecutionMode::Apply, plan, outcome);

        database.insert_command_event(&event).expect("insert event");

        assert_eq!(database.command_event_count().expect("count"), 1);
        assert!(database_path(&project_root).is_file());
    }
}
