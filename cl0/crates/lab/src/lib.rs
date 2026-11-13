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
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}
impl From<serde_json::Error> for LabError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub label: String,
    pub interventions: Vec<Intervention>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledAction {
    /// Number of successful world instruction attempts before the intervention.
    /// Actions at the same offset execute in their listed order.
    pub after_attempts: usize,
    pub action: Intervention,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledCondition {
    pub label: String,
    pub actions: Vec<ScheduledAction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochAction {
    /// Environment epoch index; independent of how many CIL instructions run.
    pub epoch: usize,
    pub action: Intervention,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochCondition {
    pub label: String,
    pub actions: Vec<EpochAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedEpochRun {
    pub id: i64,
    pub label: String,
    pub initial_sha256: String,
    pub final_sha256: String,
    pub attempts_by_epoch: Vec<usize>,
    pub world: World,
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

pub const MAX_EXPERIMENT_ATTEMPTS: usize = 1_000_000;
pub const MAX_EXPERIMENT_CONDITIONS: usize = 256;
pub const MAX_INITIAL_SNAPSHOT_BYTES: usize = 16 * 1024 * 1024;

pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl Laboratory {
    fn validate_inputs(
        initial: &World,
        conditions: usize,
        limit: usize,
        revision: &str,
    ) -> Result<(), LabError> {
        if revision.is_empty()
            || conditions == 0
            || conditions > MAX_EXPERIMENT_CONDITIONS
            || limit > MAX_EXPERIMENT_ATTEMPTS
            || initial.format_version() != 1
        {
            return Err(LabError::Integrity(
                "invalid experiment revision, format or bounds".into(),
            ));
        }
        Ok(())
    }

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
             CREATE INDEX IF NOT EXISTS idx_runs_initial ON runs(initial_sha256);
             CREATE TABLE IF NOT EXISTS scheduled_runs (
               id INTEGER PRIMARY KEY,
               schema_version INTEGER NOT NULL,
               source_revision TEXT NOT NULL,
               condition TEXT NOT NULL,
               initial_sha256 TEXT NOT NULL,
               initial_json TEXT NOT NULL,
               actions_json TEXT NOT NULL,
               attempt_limit INTEGER NOT NULL,
               attempts INTEGER NOT NULL,
               final_sha256 TEXT NOT NULL,
               final_json TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_scheduled_initial
               ON scheduled_runs(initial_sha256);
             CREATE TABLE IF NOT EXISTS scheduled_checkpoints (
               run_id INTEGER NOT NULL REFERENCES scheduled_runs(id),
               ordinal INTEGER NOT NULL,
               after_attempts INTEGER NOT NULL,
               state_sha256 TEXT NOT NULL,
               state_json TEXT NOT NULL,
               PRIMARY KEY (run_id, ordinal)
             );
             CREATE TABLE IF NOT EXISTS epoch_runs (
               id INTEGER PRIMARY KEY,
               schema_version INTEGER NOT NULL,
               source_revision TEXT NOT NULL,
               condition TEXT NOT NULL,
               initial_sha256 TEXT NOT NULL,
               initial_json TEXT NOT NULL,
               actions_json TEXT NOT NULL,
               epoch_count INTEGER NOT NULL,
               quota INTEGER NOT NULL,
               attempts_json TEXT NOT NULL,
               final_sha256 TEXT NOT NULL,
               final_json TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_epoch_runs_initial ON epoch_runs(initial_sha256);
             CREATE TABLE IF NOT EXISTS epoch_snapshots (
               run_id INTEGER NOT NULL REFERENCES epoch_runs(id),
               epoch INTEGER NOT NULL,
               state_sha256 TEXT NOT NULL,
               state_json TEXT NOT NULL,
               PRIMARY KEY (run_id, epoch)
             );",
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
        Self::validate_inputs(initial, conditions.len(), limit, source_revision)?;
        let initial_json = serde_json::to_string(initial)?;
        if initial_json.len() > MAX_INITIAL_SNAPSHOT_BYTES {
            return Err(LabError::Integrity(
                "initial snapshot exceeds size limit".into(),
            ));
        }
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
        let (
            version,
            revision,
            initial_hash,
            initial_json,
            interventions_json,
            limit,
            attempts,
            final_hash,
            final_json,
        ) = row;
        if version != 1
            || revision.is_empty()
            || limit < 0
            || attempts < 0
            || limit as u64 > MAX_EXPERIMENT_ATTEMPTS as u64
        {
            return Ok(false);
        }
        if hash_bytes(initial_json.as_bytes()) != initial_hash
            || hash_bytes(final_json.as_bytes()) != final_hash
        {
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

    fn validate_schedule(actions: &[ScheduledAction], limit: usize) -> Result<(), LabError> {
        let mut last = 0;
        for a in actions {
            if a.after_attempts < last || a.after_attempts > limit {
                return Err(LabError::Integrity(
                    "schedule offsets must be ascending and <= limit".into(),
                ));
            }
            last = a.after_attempts;
        }
        Ok(())
    }

    /// Scheduled actions are timed in *attempted CIL instructions*, not wall clock.
    /// If the world becomes inert before an action's offset, that action is NOT applied.
    fn execute_schedule(world: &mut World, actions: &[ScheduledAction], limit: usize) -> usize {
        let mut attempted = 0;
        let mut index = 0;
        while index < actions.len() {
            let at = actions[index].after_attempts;
            let actual = world.run(at - attempted);
            attempted += actual;
            if attempted != at {
                return attempted; // world could not reach the scheduled fork point
            }
            while index < actions.len() && actions[index].after_attempts == at {
                // Rejection is part of the event history and must replay identically.
                let _ = world.intervene(actions[index].action.clone());
                index += 1;
            }
        }
        attempted + world.run(limit - attempted)
    }

    /// A matched experiment may intervene after a fixed number of CIL attempts.
    /// All conditions share one byte-identical initial world. The exact pre-action
    /// world can also be recovered by running that common initial state for N steps.
    pub fn run_scheduled(
        &mut self,
        initial: &World,
        conditions: &[ScheduledCondition],
        limit: usize,
        source_revision: &str,
    ) -> Result<Vec<RecordedRun>, LabError> {
        Self::validate_inputs(initial, conditions.len(), limit, source_revision)?;
        for c in conditions {
            Self::validate_schedule(&c.actions, limit)?;
        }
        let initial_json = serde_json::to_string(initial)?;
        if initial_json.len() > MAX_INITIAL_SNAPSHOT_BYTES {
            return Err(LabError::Integrity(
                "initial snapshot exceeds size limit".into(),
            ));
        }
        let initial_sha256 = hash_bytes(initial_json.as_bytes());
        let transaction = self.conn.transaction()?;
        let mut runs = Vec::with_capacity(conditions.len());
        for condition in conditions {
            let mut world: World = serde_json::from_str(&initial_json)?;
            let attempts = Self::execute_schedule(&mut world, &condition.actions, limit);
            let final_json = serde_json::to_string(&world)?;
            let final_sha256 = hash_bytes(final_json.as_bytes());
            let actions_json = serde_json::to_string(&condition.actions)?;
            transaction.execute(
                "INSERT INTO scheduled_runs
                 (schema_version,source_revision,condition,initial_sha256,initial_json,actions_json,
                  attempt_limit,attempts,final_sha256,final_json)
                 VALUES (1,?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![
                    source_revision,
                    condition.label,
                    initial_sha256,
                    initial_json,
                    actions_json,
                    limit as i64,
                    attempts as i64,
                    final_sha256,
                    final_json
                ],
            )?;
            // Persist each reached pre-intervention fork state, including prior same-offset actions.
            let run_id = transaction.last_insert_rowid();
            let mut checkpoint_world: World = serde_json::from_str(&initial_json)?;
            let mut completed = 0;
            for (ordinal, action) in condition.actions.iter().enumerate() {
                completed += checkpoint_world.run(action.after_attempts - completed);
                if completed != action.after_attempts {
                    break;
                }
                let snapshot = serde_json::to_string(&checkpoint_world)?;
                let digest = hash_bytes(snapshot.as_bytes());
                transaction.execute(
                    "INSERT INTO scheduled_checkpoints
                     (run_id, ordinal, after_attempts, state_sha256, state_json)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![run_id, ordinal as i64, completed as i64, digest, snapshot],
                )?;
                let _ = checkpoint_world.intervene(action.action.clone());
            }
            runs.push(RecordedRun {
                id: run_id,
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

    /// Exact same-version replay including scheduled events and their timestamps.
    pub fn verify_scheduled_replay(&self, id: i64) -> Result<bool, LabError> {
        let row = self.conn.query_row(
            "SELECT schema_version,source_revision,initial_sha256,initial_json,
                    actions_json,attempt_limit,attempts,final_sha256,final_json
             FROM scheduled_runs WHERE id=?1",
            [id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, String>(8)?,
                ))
            },
        )?;
        let (
            version,
            revision,
            initial_hash,
            initial_json,
            actions_json,
            limit,
            attempts,
            final_hash,
            final_json,
        ) = row;
        if version != 1 || revision.is_empty() || limit < 0 || attempts < 0 {
            return Ok(false);
        }
        if hash_bytes(initial_json.as_bytes()) != initial_hash
            || hash_bytes(final_json.as_bytes()) != final_hash
        {
            return Ok(false);
        }
        let mut world: World = serde_json::from_str(&initial_json)?;
        let actions: Vec<ScheduledAction> = serde_json::from_str(&actions_json)?;
        if Self::validate_schedule(&actions, limit as usize).is_err() {
            return Ok(false);
        }
        let actual = Self::execute_schedule(&mut world, &actions, limit as usize);
        if actual != attempts as usize || serde_json::to_string(&world)? != final_json {
            return Ok(false);
        }
        let mut expected = Vec::new();
        let mut fork = serde_json::from_str::<World>(&initial_json)?;
        let mut performed = 0;
        for (ordinal, action) in actions.iter().enumerate() {
            performed += fork.run(action.after_attempts - performed);
            if performed != action.after_attempts {
                break;
            }
            let json = serde_json::to_string(&fork)?;
            expected.push((
                ordinal as i64,
                performed as i64,
                hash_bytes(json.as_bytes()),
                json,
            ));
            let _ = fork.intervene(action.action.clone());
        }
        let mut query = self.conn.prepare(
            "SELECT ordinal,after_attempts,state_sha256,state_json
             FROM scheduled_checkpoints WHERE run_id=?1 ORDER BY ordinal",
        )?;
        let saved: Vec<(i64, i64, String, String)> = query
            .query_map([id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<_, _>>()?;
        Ok(saved == expected)
    }

    /// Load a durable *pre-intervention* world to fork a new experiment.
    /// A checkpoint's index is the action ordinal, including ties at the same offset.
    pub fn load_scheduled_checkpoint(&self, run_id: i64, ordinal: i64) -> Result<World, LabError> {
        let (digest, json): (String, String) = self.conn.query_row(
            "SELECT state_sha256,state_json FROM scheduled_checkpoints
             WHERE run_id=?1 AND ordinal=?2",
            params![run_id, ordinal],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        if hash_bytes(json.as_bytes()) != digest {
            return Err(LabError::Integrity("checkpoint digest mismatch".into()));
        }
        Ok(serde_json::from_str(&json)?)
    }

    fn validate_epoch_plan(
        actions: &[EpochAction],
        epochs: usize,
    ) -> Result<(), LabError> {
        let mut previous = 0;
        for item in actions {
            if item.epoch >= epochs || item.epoch < previous {
                return Err(LabError::Integrity("invalid or unsorted epoch action".into()));
            }
            previous = item.epoch;
        }
        if actions.len() > 1024 {
            return Err(LabError::Integrity("too many epoch actions".into()));
        }
        Ok(())
    }

    /// The environment advances independently from CIL progress. Every epoch
    /// runs its external actions even if the world has become inert.
    fn execute_epochs(
        world: &mut World,
        epochs: usize,
        quota: usize,
        actions: &[EpochAction],
    ) -> (Vec<usize>, Vec<(String, String)>) {
        let mut index = 0;
        let mut attempts = Vec::with_capacity(epochs);
        let mut snapshots = Vec::with_capacity(epochs);
        for epoch in 0..epochs {
            while index < actions.len() && actions[index].epoch == epoch {
                let _ = world.intervene(actions[index].action.clone());
                index += 1;
            }
            attempts.push(world.run(quota));
            let json = serde_json::to_string(world).expect("world snapshot serializable");
            snapshots.push((hash_bytes(json.as_bytes()), json));
        }
        (attempts, snapshots)
    }

    /// Experimental epochs are external observation opportunities, not world
    /// instruction attempts. The same initial serialized world is forked for
    /// every matched condition. All post-epoch states are persisted.
    pub fn run_epochs(
        &mut self,
        initial: &World,
        conditions: &[EpochCondition],
        epochs: usize,
        quota: usize,
        source_revision: &str,
    ) -> Result<Vec<RecordedEpochRun>, LabError> {
        if epochs == 0 || epochs > 128 || quota == 0 {
            return Err(LabError::Integrity("invalid epoch count or quota".into()));
        }
        let limit = epochs
            .checked_mul(quota)
            .ok_or_else(|| LabError::Integrity("epoch attempt bound overflow".into()))?;
        Self::validate_inputs(initial, conditions.len(), limit, source_revision)?;
        for condition in conditions {
            Self::validate_epoch_plan(&condition.actions, epochs)?;
        }
        let initial_json = serde_json::to_string(initial)?;
        if initial_json.len() > MAX_INITIAL_SNAPSHOT_BYTES {
            return Err(LabError::Integrity("initial snapshot exceeds size limit".into()));
        }
        let initial_sha256 = hash_bytes(initial_json.as_bytes());
        let transaction = self.conn.transaction()?;
        let mut results = Vec::with_capacity(conditions.len());
        for condition in conditions {
            let mut world: World = serde_json::from_str(&initial_json)?;
            let (attempts_by_epoch, snapshots) =
                Self::execute_epochs(&mut world, epochs, quota, &condition.actions);
            let (final_sha256, final_json) = snapshots
                .last()
                .expect("nonzero epochs")
                .clone();
            transaction.execute(
                "INSERT INTO epoch_runs (
                    schema_version,source_revision,condition,initial_sha256,
                    initial_json,actions_json,epoch_count,quota,attempts_json,
                    final_sha256,final_json
                 ) VALUES (1,?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                params![
                    source_revision,
                    condition.label,
                    initial_sha256,
                    initial_json,
                    serde_json::to_string(&condition.actions)?,
                    epochs as i64,
                    quota as i64,
                    serde_json::to_string(&attempts_by_epoch)?,
                    final_sha256,
                    final_json,
                ],
            )?;
            let run_id = transaction.last_insert_rowid();
            for (epoch, (sha, json)) in snapshots.iter().enumerate() {
                transaction.execute(
                    "INSERT INTO epoch_snapshots (run_id,epoch,state_sha256,state_json)
                     VALUES (?1,?2,?3,?4)",
                    params![run_id, epoch as i64, sha, json],
                )?;
            }
            results.push(RecordedEpochRun {
                id: run_id,
                label: condition.label.clone(),
                initial_sha256: initial_sha256.clone(),
                final_sha256,
                attempts_by_epoch,
                world,
            });
        }
        transaction.commit()?;
        Ok(results)
    }

    /// Read-only same-revision recomputation including every environmental
    /// action and every post-epoch world state (all events and resources).
    pub fn verify_epoch_replay(&self, id: i64) -> Result<bool, LabError> {
        let row = self.conn.query_row(
            "SELECT schema_version,source_revision,initial_sha256,initial_json,
                    actions_json,epoch_count,quota,attempts_json,final_sha256,final_json
             FROM epoch_runs WHERE id=?1",
            [id],
            |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, String>(4)?,
                    r.get::<_, i64>(5)?,
                    r.get::<_, i64>(6)?,
                    r.get::<_, String>(7)?,
                    r.get::<_, String>(8)?,
                    r.get::<_, String>(9)?,
                ))
            },
        )?;
        let (version, revision, initial_sha, initial_json, actions_json,
            epochs, quota, attempts_json, final_sha, final_json) = row;
        if version != 1 || revision.is_empty() || epochs <= 0 || epochs > 128 || quota <= 0
            || (epochs as usize).checked_mul(quota as usize)
                .is_none_or(|n| n > MAX_EXPERIMENT_ATTEMPTS)
            || hash_bytes(initial_json.as_bytes()) != initial_sha
            || hash_bytes(final_json.as_bytes()) != final_sha
        {
            return Ok(false);
        }
        let actions: Vec<EpochAction> = serde_json::from_str(&actions_json)?;
        if Self::validate_epoch_plan(&actions, epochs as usize).is_err() {
            return Ok(false);
        }
        let recorded: Vec<usize> = serde_json::from_str(&attempts_json)?;
        let mut world: World = serde_json::from_str(&initial_json)?;
        let (attempts, snapshots) =
            Self::execute_epochs(&mut world, epochs as usize, quota as usize, &actions);
        if attempts != recorded || snapshots.last().is_none_or(|(sha, json)| {
            *sha != final_sha || *json != final_json
        }) {
            return Ok(false);
        }
        let mut query = self.conn.prepare(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots
             WHERE run_id=?1 ORDER BY epoch",
        )?;
        let stored: Vec<(i64, String, String)> = query
            .query_map([id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<_, _>>()?;
        Ok(stored.len() == epochs as usize
            && stored.iter().enumerate().all(|(epoch, (number, sha, json))| {
                *number == epoch as i64
                    && snapshots[epoch].0 == *sha
                    && snapshots[epoch].1 == *json
                    && hash_bytes(json.as_bytes()) == *sha
            }))
    }

    pub fn scheduled_count(&self) -> Result<i64, LabError> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM scheduled_runs", [], |row| row.get(0))?)
    }

    pub fn count(&self) -> Result<i64, LabError> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM runs", [], |row| row.get(0))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cl0_world::{Op, Process};

    fn fixture() -> World {
        let mut world = World::new(1, 8);
        world
            .load(Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 5,
                        next: 1,
                    },
                    Op::Const {
                        dst: 1,
                        value: 91,
                        next: 2,
                    },
                    Op::Write {
                        address: 0,
                        value: 1,
                        next: 3,
                    },
                    Op::Halt,
                ],
                10,
            ))
            .unwrap();
        world
    }

    #[test]
    fn external_epochs_arrive_even_after_world_becomes_inert() {
        let mut laboratory = Laboratory::in_memory().unwrap();
        let conditions = [
            EpochCondition { label: "untreated".into(), actions: vec![] },
            EpochCondition { label: "pulsed".into(), actions: vec![
                EpochAction { epoch: 0, action: Intervention::SetComputePool { credits: 3 } },
                EpochAction { epoch: 2, action: Intervention::SetComputePool { credits: 7 } },
            ] },
        ];
        let mut initial = fixture();
        initial.intervene(Intervention::SetCredits { process: 1, credits: 0 }).unwrap();
        let results = laboratory.run_epochs(&initial, &conditions, 4, 3, "epoch-test").unwrap();
        assert_eq!(results[0].attempts_by_epoch, vec![0, 0, 0, 0]);
        assert_eq!(results[1].attempts_by_epoch, vec![0, 0, 0, 0]);
        assert_eq!(results[1].world.available_compute_pool(), 7);
        assert_eq!(results[1].world.events().iter().filter(|event| {
            event.origin == cl0_world::Origin::External
                && matches!(&event.kind, cl0_world::EventKind::Intervened(text)
                    if text.contains("SetComputePool"))
        }).count(), 2);
        for result in results {
            assert!(laboratory.verify_epoch_replay(result.id).unwrap());
        }
    }

    #[test]
    fn epoch_plans_reject_unsorted_and_out_of_range_before_persisting() {
        let mut lab = Laboratory::in_memory().unwrap();
        let bad = [EpochCondition { label: "bad".into(), actions: vec![
            EpochAction { epoch: 2, action: Intervention::Disable { process: 1 } },
            EpochAction { epoch: 1, action: Intervention::Disable { process: 1 } },
        ] }];
        assert!(lab.run_epochs(&fixture(), &bad, 4, 2, "epoch-test").is_err());
        let bad = [EpochCondition { label: "late".into(), actions: vec![
            EpochAction { epoch: 4, action: Intervention::Disable { process: 1 } }
        ] }];
        assert!(lab.run_epochs(&fixture(), &bad, 4, 2, "epoch-test").is_err());
        let existing: i64 = lab.conn.query_row("SELECT COUNT(*) FROM epoch_runs", [],
            |r| r.get(0)).unwrap();
        assert_eq!(existing, 0);
    }

    #[test]
    fn epoch_snapshot_and_final_state_tampering_is_detected() {
        let lab = Laboratory::in_memory().unwrap();
        let mut lab = lab;
        let results = lab.run_epochs(&fixture(), &[
            EpochCondition { label: "control".into(), actions: vec![] }
        ], 3, 2, "epoch-test").unwrap();
        let id = results[0].id;
        assert!(lab.verify_epoch_replay(id).unwrap());
        lab.conn.execute(
            "UPDATE epoch_snapshots SET state_json='{}' WHERE run_id=?1 AND epoch=1",
            [id],
        ).unwrap();
        assert!(!lab.verify_epoch_replay(id).unwrap());
    }

    #[test]
    fn matched_runs_share_initial_snapshot_and_have_causal_difference() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab
            .run_matched(
                &fixture(),
                &[
                    Condition {
                        label: "control".into(),
                        interventions: vec![],
                    },
                    Condition {
                        label: "disable".into(),
                        interventions: vec![Intervention::Disable { process: 1 }],
                    },
                ],
                20,
                "test-revision",
            )
            .unwrap();
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
        let outcomes = lab
            .run_matched(
                &fixture(),
                &[Condition {
                    label: "reject-capacity".into(),
                    interventions: vec![
                        Intervention::SetCell { key: 2, value: 1 },
                        Intervention::SetCell { key: 3, value: 2 },
                    ],
                }],
                0,
                "test-revision",
            )
            .unwrap();
        assert_eq!(outcomes[0].world.used_cells(), 1);
        assert!(lab.verify_replay(outcomes[0].id).unwrap());
    }

    #[test]
    fn record_tampering_is_detected_on_replay() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab
            .run_matched(
                &fixture(),
                &[Condition {
                    label: "control".into(),
                    interventions: vec![],
                }],
                10,
                "test-revision",
            )
            .unwrap();
        let id = outcomes[0].id;
        assert!(lab.verify_replay(id).unwrap());
        lab.conn
            .execute("UPDATE runs SET final_json='{}' WHERE id=?1", [id])
            .unwrap();
        assert!(!lab.verify_replay(id).unwrap());
    }

    #[test]
    fn disk_database_reopens_and_replays() {
        let directory = tempfile::tempdir().unwrap();
        let db = directory.path().join("cl0.sqlite");
        let id;
        {
            let mut lab = Laboratory::open(&db).unwrap();
            let records = lab
                .run_matched(
                    &fixture(),
                    &[
                        Condition {
                            label: "unmodified".into(),
                            interventions: vec![],
                        },
                        Condition {
                            label: "zero-budget".into(),
                            interventions: vec![Intervention::SetCredits {
                                process: 1,
                                credits: 0,
                            }],
                        },
                    ],
                    20,
                    "test-revision",
                )
                .unwrap();
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
        world
            .load(Process::new(
                5,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 0,
                        next: 1,
                    },
                    Op::CloneNode {
                        dst: 1,
                        source: 0,
                        next: 2,
                    },
                    Op::Halt,
                ],
                5,
            ))
            .unwrap();
        let mut lab = Laboratory::in_memory().unwrap();
        let records = lab
            .run_matched(
                &world,
                &[
                    Condition {
                        label: "allocation".into(),
                        interventions: vec![],
                    },
                    Condition {
                        label: "zero-compute".into(),
                        interventions: vec![Intervention::SetCredits {
                            process: 5,
                            credits: 0,
                        }],
                    },
                ],
                8,
                "stage3-test",
            )
            .unwrap();
        assert_eq!(records[0].world.occupied_units(), 4);
        assert_eq!(records[1].world.occupied_units(), 3);
        assert_eq!(records[0].attempts, 3);
        assert_eq!(records[1].attempts, 0);
        assert!(records.iter().all(|run| lab.verify_replay(run.id).unwrap()));
    }

    #[test]
    fn graph_object_launch_replays_with_zero_credit_control() {
        let mut initial = World::with_total_capacity(0, 16, 16);
        initial
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 7,
                        next: 2,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 1,
                        dst: 2,
                        next: 3,
                    },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 4,
                    },
                    Op::GraphLaunch {
                        graph: 0,
                        credits: 1,
                        dst: 2,
                        next: 5,
                    },
                    Op::Halt,
                    Op::Halt,
                    Op::Halt,
                ],
                12,
            ))
            .unwrap();
        let mut lab = Laboratory::in_memory().unwrap();
        let runs = lab
            .run_matched(
                &initial,
                &[
                    Condition {
                        label: "constructed-runner".into(),
                        interventions: vec![],
                    },
                    Condition {
                        label: "no-computation".into(),
                        interventions: vec![Intervention::SetCredits {
                            process: 1,
                            credits: 0,
                        }],
                    },
                ],
                20,
                "stage-3.2-test",
            )
            .unwrap();
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
    fn constructed_instruction_descriptor_replays_with_negative_control() {
        let mut initial = World::with_total_capacity(8, 16, 24);
        initial
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 100,
                        next: 2,
                    },
                    Op::GraphEncode {
                        graph: 0,
                        address: 1,
                        dst: 2,
                        next: 3,
                    },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 4,
                    },
                    Op::GraphLaunch {
                        graph: 0,
                        credits: 1,
                        dst: 2,
                        next: 5,
                    },
                    Op::Halt,
                ],
                12,
            ))
            .unwrap();
        for (offset, value) in [19, 0, 0, 0, 0].into_iter().enumerate() {
            initial
                .intervene(Intervention::SetCell {
                    key: 100 + offset as i64,
                    value,
                })
                .unwrap();
        }
        let mut lab = Laboratory::in_memory().unwrap();
        let cases = [
            Condition {
                label: "described-program".into(),
                interventions: vec![],
            },
            Condition {
                label: "no-computation".into(),
                interventions: vec![Intervention::SetCredits {
                    process: 1,
                    credits: 0,
                }],
            },
        ];
        let outcomes = lab
            .run_matched(&initial, &cases, 32, "stage3.3-test")
            .unwrap();
        assert_eq!(outcomes[0].initial_sha256, outcomes[1].initial_sha256);
        assert!(outcomes[0].world.process(2).unwrap().stopped);
        assert_eq!(outcomes[1].world.process(2), None);
        assert_eq!(outcomes[0].world.remaining_credits(), 5);
        assert_eq!(outcomes[1].attempts, 0);
        for run in outcomes {
            assert!(lab.verify_replay(run.id).unwrap());
        }
    }

    #[test]
    fn timed_intervention_changes_only_later_execution() {
        let initial = fixture();
        let mut lab = Laboratory::in_memory().unwrap();
        let matched = lab
            .run_scheduled(
                &initial,
                &[
                    ScheduledCondition {
                        label: "control".into(),
                        actions: vec![],
                    },
                    ScheduledCondition {
                        label: "disable-before-write".into(),
                        actions: vec![ScheduledAction {
                            after_attempts: 2,
                            action: Intervention::Disable { process: 1 },
                        }],
                    },
                ],
                8,
                "stage3.4-timing",
            )
            .unwrap();
        assert_eq!(matched[0].initial_sha256, matched[1].initial_sha256);
        assert_eq!(matched[0].world.cell(5), Some(91));
        assert_eq!(matched[1].world.cell(5), None);
        assert_eq!(matched[1].attempts, 2);
        assert!(matched
            .iter()
            .all(|r| lab.verify_scheduled_replay(r.id).unwrap()));
    }

    #[test]
    fn missed_event_and_bad_schedule_do_not_apply_silently() {
        let mut lab = Laboratory::in_memory().unwrap();
        let initial = fixture();
        let missed = lab
            .run_scheduled(
                &initial,
                &[ScheduledCondition {
                    label: "too-late".into(),
                    actions: vec![ScheduledAction {
                        after_attempts: 6,
                        action: Intervention::SetCell { key: 99, value: 1 },
                    }],
                }],
                8,
                "stage3.4-timing",
            )
            .unwrap();
        assert_eq!(missed[0].attempts, 4);
        assert_eq!(missed[0].world.cell(99), None);
        assert!(lab.verify_scheduled_replay(missed[0].id).unwrap());
        let result = lab.run_scheduled(
            &initial,
            &[ScheduledCondition {
                label: "bad-order".into(),
                actions: vec![
                    ScheduledAction {
                        after_attempts: 3,
                        action: Intervention::Disable { process: 1 },
                    },
                    ScheduledAction {
                        after_attempts: 2,
                        action: Intervention::Disable { process: 1 },
                    },
                ],
            }],
            8,
            "stage3.4-timing",
        );
        assert!(result.is_err());
        assert_eq!(lab.scheduled_count().unwrap(), 1);
    }

    #[test]
    fn scheduled_run_reopens_and_detects_tampering() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("scheduled.sqlite");
        let id;
        {
            let mut lab = Laboratory::open(&path).unwrap();
            let outcomes = lab
                .run_scheduled(
                    &fixture(),
                    &[ScheduledCondition {
                        label: "set-data".into(),
                        actions: vec![ScheduledAction {
                            after_attempts: 1,
                            action: Intervention::SetCell { key: 5, value: 77 },
                        }],
                    }],
                    8,
                    "stage3.4-timing",
                )
                .unwrap();
            id = outcomes[0].id;
            assert_eq!(outcomes[0].world.cell(5), Some(91));
        }
        let lab = Laboratory::open(&path).unwrap();
        assert!(lab.verify_scheduled_replay(id).unwrap());
        lab.conn
            .execute(
                "UPDATE scheduled_runs SET final_json='{}' WHERE id=?1",
                [id],
            )
            .unwrap();
        assert!(!lab.verify_scheduled_replay(id).unwrap());
    }

    #[test]
    fn checkpoints_preserve_exact_pre_intervention_fork_and_detect_tampering() {
        let mut lab = Laboratory::in_memory().unwrap();
        let runs = lab
            .run_scheduled(
                &fixture(),
                &[ScheduledCondition {
                    label: "snapshot".into(),
                    actions: vec![ScheduledAction {
                        after_attempts: 2,
                        action: Intervention::Disable { process: 1 },
                    }],
                }],
                8,
                "checkpoint-test",
            )
            .unwrap();
        let id = runs[0].id;
        let checkpoint = lab.load_scheduled_checkpoint(id, 0).unwrap();
        assert_eq!(checkpoint.process(1).unwrap().pc, 2);
        assert_eq!(checkpoint.process(1).unwrap().registers[1], 91);
        assert_eq!(checkpoint.cell(5), None);
        assert_eq!(checkpoint.events().len(), 3); // external load + two internal CIL steps
        assert!(lab.verify_scheduled_replay(id).unwrap());
        lab.conn
            .execute(
                "UPDATE scheduled_checkpoints SET state_sha256='corrupt' WHERE run_id=?1",
                [id],
            )
            .unwrap();
        assert!(!lab.verify_scheduled_replay(id).unwrap());
        assert!(lab.load_scheduled_checkpoint(id, 0).is_err());
    }

    #[test]
    fn over_budget_experiment_is_rejected_before_sqlite_write() {
        let mut lab = Laboratory::in_memory().unwrap();
        let conditions = [ScheduledCondition {
            label: "bounded".into(),
            actions: vec![],
        }];
        let result = lab.run_scheduled(
            &fixture(),
            &conditions,
            MAX_EXPERIMENT_ATTEMPTS + 1,
            "safety-test",
        );
        assert!(result.is_err());
        assert_eq!(lab.scheduled_count().unwrap(), 0);
    }

    #[test]
    fn world_snapshots_preserve_explicit_version() {
        let world = World::with_strict_capacity(2, 8, 10);
        let encoded = serde_json::to_string(&world).unwrap();
        let value: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(value["format_version"], 1);
        let reconstructed: World = serde_json::from_str(&encoded).unwrap();
        assert_eq!(reconstructed, world);
        assert_eq!(reconstructed.format_version(), 1);
    }

    #[test]
    fn empty_revision_and_conditions_are_rejected() {
        let mut lab = Laboratory::in_memory().unwrap();
        assert!(lab
            .run_matched(&fixture(), &[], 10, "test-revision")
            .is_err());
        assert!(lab
            .run_matched(
                &fixture(),
                &[Condition {
                    label: "x".into(),
                    interventions: vec![]
                }],
                10,
                ""
            )
            .is_err());
        assert_eq!(lab.count().unwrap(), 0);
    }
}
