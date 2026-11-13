//! External research instrumentation. None of these functions are visible to CIL.
use cl0_world::{Intervention, World};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{error::Error, fmt, path::Path};

#[derive(Debug)]
pub enum LabError {
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
    Integrity(String),
}

impl fmt::Display for LabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(e) => write!(f, "SQLite: {e}"),
            Self::Json(e) => write!(f, "JSON: {e}"),
            Self::Integrity(e) => write!(f, "integrity: {e}"),
        }
    }
}
impl Error for LabError {}
impl From<rusqlite::Error> for LabError {
    fn from(value: rusqlite::Error) -> Self { Self::Sqlite(value) }
}
impl From<serde_json::Error> for LabError {
    fn from(value: serde_json::Error) -> Self { Self::Json(value) }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub label: String,
    pub interventions: Vec<Intervention>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedRun {
    pub id: i64,
    pub label: String,
    pub initial_sha256: String,
    pub final_sha256: String,
    pub attempts: usize,
    pub world: World,
}

pub struct Laboratory {
    conn: Connection,
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl Laboratory {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LabError> {
        Self::from_connection(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self, LabError> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(conn: Connection) -> Result<Self, LabError> {
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS runs (
               id INTEGER PRIMARY KEY,
               schema_version INTEGER NOT NULL,
               source_revision TEXT NOT NULL,
               condition TEXT NOT NULL,
               initial_sha256 TEXT NOT NULL,
               initial_json TEXT NOT NULL,
               interventions_json TEXT NOT NULL,
               attempt_limit INTEGER NOT NULL,
               attempts INTEGER NOT NULL,
               final_sha256 TEXT NOT NULL,
               final_json TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_runs_initial ON runs(initial_sha256);",
        )?;
        Ok(Self { conn })
    }

    /// Condition order is a record of experimental order, not a world stimulus.
    /// Every run is based on the exact same pre-intervention snapshot.
    /// Rejected interventions are recorded; a rejected action is not silently applied.
    pub fn run_matched(
        &mut self,
        initial: &World,
        conditions: &[Condition],
        limit: usize,
        source_revision: &str,
    ) -> Result<Vec<RecordedRun>, LabError> {
        if source_revision.is_empty() {
            return Err(LabError::Integrity("missing source revision".into()));
        }
        if conditions.is_empty() {
            return Err(LabError::Integrity("no experimental conditions".into()));
        }
        let initial_json = serde_json::to_string(initial)?;
        let initial_sha256 = hash_bytes(initial_json.as_bytes());
        let transaction = self.conn.transaction()?;
        let mut runs = Vec::with_capacity(conditions.len());
        for condition in conditions {
            let mut world: World = serde_json::from_str(&initial_json)?;
            for intervention in &condition.interventions {
                // Rejections are part of the experiment and can be audited.
                let _ = world.intervene(intervention.clone());
            }
            let attempts = world.run(limit);
            let final_json = serde_json::to_string(&world)?;
            let final_sha256 = hash_bytes(final_json.as_bytes());
            let interventions_json = serde_json::to_string(&condition.interventions)?;
            transaction.execute(
                "INSERT INTO runs (schema_version,source_revision,condition,initial_sha256,initial_json,interventions_json,attempt_limit,attempts,final_sha256,final_json)
                 VALUES (1,?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![
                    source_revision, condition.label, initial_sha256, initial_json,
                    interventions_json, limit as i64, attempts as i64,
                    final_sha256, final_json
                ],
            )?;
            runs.push(RecordedRun {
                id: transaction.last_insert_rowid(),
                label: condition.label.clone(),
                initial_sha256: initial_sha256.clone(),
                final_sha256,
                attempts,
                world,
            });
        }
        transaction.commit()?;
        Ok(runs)
    }

    /// Rebuild the exact world state from its saved baseline and interventions;
    /// compare full event history and final state via canonical JSON and SHA-256.
    pub fn verify_replay(&self, id: i64) -> Result<bool, LabError> {
        let row = self.conn.query_row(
            "SELECT schema_version,source_revision,initial_sha256,initial_json,interventions_json,attempt_limit,attempts,final_sha256,final_json
             FROM runs WHERE id=?1", [id],
            |row| Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
            )),
        )?;
        let (version, revision, initial_hash, initial_json, interventions_json, limit, attempts, final_hash, final_json) = row;
        if version != 1 || revision.is_empty() || limit < 0 || attempts < 0 {
            return Ok(false);
        }
        if hash_bytes(initial_json.as_bytes()) != initial_hash || hash_bytes(final_json.as_bytes()) != final_hash {
            return Ok(false);
        }
        let mut world: World = serde_json::from_str(&initial_json)?;
        let interventions: Vec<Intervention> = serde_json::from_str(&interventions_json)?;
        for intervention in interventions {
            let _ = world.intervene(intervention);
        }
        let observed = world.run(limit as usize);
        let reconstructed = serde_json::to_string(&world)?;
        Ok(observed == attempts as usize && reconstructed == final_json)
    }

    pub fn count(&self) -> Result<i64, LabError> {
        Ok(self.conn.query_row("SELECT COUNT(*) FROM runs", [], |row| row.get(0))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cl0_world::{Op, Process};

    fn fixture() -> World {
        let mut world = World::new(1, 8);
        world.load(Process::new(1, vec![
            Op::Const { dst: 0, value: 5, next: 1 },
            Op::Const { dst: 1, value: 91, next: 2 },
            Op::Write { address: 0, value: 1, next: 3 },
            Op::Halt,
        ], 10)).unwrap();
        world
    }

    #[test]
    fn matched_runs_share_initial_snapshot_and_have_causal_difference() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab.run_matched(&fixture(), &[
            Condition { label: "control".into(), interventions: vec![] },
            Condition { label: "disable".into(), interventions: vec![Intervention::Disable { process: 1 }] },
        ], 20, "test-revision").unwrap();
        assert_eq!(outcomes[0].initial_sha256, outcomes[1].initial_sha256);
        assert_eq!(outcomes[0].world.cell(5), Some(91));
        assert_eq!(outcomes[1].world.cell(5), None);
        assert_ne!(outcomes[0].final_sha256, outcomes[1].final_sha256);
        assert_eq!(lab.count().unwrap(), 2);
        for outcome in &outcomes {
            assert!(lab.verify_replay(outcome.id).unwrap());
        }
    }

    #[test]
    fn rejected_intervention_replays_without_hidden_mutation() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab.run_matched(&fixture(), &[
            Condition { label: "reject-capacity".into(), interventions: vec![
                Intervention::SetCell { key: 2, value: 1 },
                Intervention::SetCell { key: 3, value: 2 },
            ] },
        ], 0, "test-revision").unwrap();
        assert_eq!(outcomes[0].world.used_cells(), 1);
        assert!(lab.verify_replay(outcomes[0].id).unwrap());
    }

    #[test]
    fn record_tampering_is_detected_on_replay() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab.run_matched(&fixture(), &[
            Condition { label: "control".into(), interventions: vec![] },
        ], 10, "test-revision").unwrap();
        let id = outcomes[0].id;
        assert!(lab.verify_replay(id).unwrap());
        lab.conn.execute("UPDATE runs SET final_json='{}' WHERE id=?1", [id]).unwrap();
        assert!(!lab.verify_replay(id).unwrap());
    }

    #[test]
    fn disk_database_reopens_and_replays() {
        let directory = tempfile::tempdir().unwrap();
        let db = directory.path().join("cl0.sqlite");
        let id;
        {
            let mut lab = Laboratory::open(&db).unwrap();
            let records = lab.run_matched(&fixture(), &[
                Condition { label: "unmodified".into(), interventions: vec![] },
                Condition { label: "zero-budget".into(), interventions: vec![
                    Intervention::SetCredits { process: 1, credits: 0 }
                ] },
            ], 20, "test-revision").unwrap();
            id = records[0].id;
            assert_eq!(records[1].attempts, 0);
            assert_eq!(records[1].world.cell(5), None);
        }
        let reopened = Laboratory::open(&db).unwrap();
        assert_eq!(reopened.count().unwrap(), 2);
        assert!(reopened.verify_replay(id).unwrap());
        assert!(reopened.verify_replay(id + 1).unwrap());
    }

    #[test]
    fn replay_includes_resource_priced_code_allocation() {
        let mut world = World::with_total_capacity(0, 8, 4);
        world.load(Process::new(5, vec![
            Op::Const { dst: 0, value: 0, next: 1 },
            Op::CloneNode { dst: 1, source: 0, next: 2 },
            Op::Halt,
        ], 5)).unwrap();
        let mut lab = Laboratory::in_memory().unwrap();
        let records = lab.run_matched(&world, &[
            Condition { label: "allocation".into(), interventions: vec![] },
            Condition { label: "zero-compute".into(), interventions: vec![
                Intervention::SetCredits { process: 5, credits: 0 }
            ] },
        ], 8, "stage3-test").unwrap();
        assert_eq!(records[0].world.occupied_units(), 4);
        assert_eq!(records[1].world.occupied_units(), 3);
        assert_eq!(records[0].attempts, 3);
        assert_eq!(records[1].attempts, 0);
        assert!(records.iter().all(|run| lab.verify_replay(run.id).unwrap()));
    }

    #[test]
    fn graph_object_launch_replays_with_zero_credit_control() {
        let mut initial = World::with_total_capacity(0, 16, 16);
        initial.load(Process::new(1, vec![
            Op::GraphNew { dst: 0, next: 1 },
            Op::Const { dst: 1, value: 7, next: 2 },
            Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next: 3 },
            Op::Const { dst: 1, value: 2, next: 4 },
            Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 5 },
            Op::Halt,
            Op::Halt,
            Op::Halt,
        ], 12)).unwrap();
        let mut lab = Laboratory::in_memory().unwrap();
        let runs = lab.run_matched(&initial, &[
            Condition { label: "constructed-runner".into(), interventions: vec![] },
            Condition { label: "no-computation".into(), interventions: vec![
                Intervention::SetCredits { process: 1, credits: 0 },
            ] },
        ], 20, "stage-3.2-test").unwrap();
        assert_eq!(runs[0].world.graph_count(), 0);
        assert!(runs[0].world.process(2).is_some());
        assert!(runs[0].world.process(2).unwrap().stopped);
        assert_eq!(runs[0].world.occupied_units(), 9);
        assert_eq!(runs[1].world.graph_count(), 0);
        assert!(runs[1].world.process(2).is_none());
        assert_eq!(runs[1].world.occupied_units(), 8);
        for run in runs {
            assert!(lab.verify_replay(run.id).unwrap());
        }
    }

    #[test]
    fn empty_revision_and_conditions_are_rejected() {
        let mut lab = Laboratory::in_memory().unwrap();
        assert!(lab.run_matched(&fixture(), &[], 10, "test-revision").is_err());
        assert!(lab.run_matched(&fixture(), &[Condition { label: "x".into(), interventions: vec![] }], 10, "").is_err());
        assert_eq!(lab.count().unwrap(), 0);
    }
}
