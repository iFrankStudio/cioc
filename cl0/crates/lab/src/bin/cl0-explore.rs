//! Pre-registered exploratory baseline; seeded CIL is investigator-generated, NOT discovered life.
use cl0_lab::{Laboratory, ScheduledAction, ScheduledCondition};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

fn next_random(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}

/// The same seed constructs the same externally authored random CIL baseline.
/// No reward, repair, reproduction, selection, or survival check appears here.
fn seed_world(seed: u64) -> World {
    let mut state = seed;
    let mut nodes = vec![
        Op::Const { dst: 0, value: (next_random(&mut state) % 17) as i64, next: 1 },
        Op::Const { dst: 1, value: (next_random(&mut state) % 101) as i64, next: 2 },
    ];
    for i in 2..11 {
        let next = i + 1;
        let op = match next_random(&mut state) % 6 {
            0 => Op::Add { dst: 1, lhs: 1, rhs: 0, next },
            1 => Op::Sub { dst: 1, lhs: 1, rhs: 0, next },
            2 => Op::Write { address: 0, value: 1, next },
            3 => Op::Read { dst: 2, address: 0, next },
            4 => Op::DrawCompute { amount: 0, next },
            _ => Op::Branch { test: 1, nonzero: next, zero: i - 1 },
        };
        nodes.push(op);
    }
    nodes.push(Op::Halt);
    let mut world = World::with_strict_capacity(24, 32, 64);
    world.load(Process::new(1, nodes, 48)).expect("generated baseline fits");
    world
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "cl0-explore.sqlite".into());
    let replicas: usize = args.next().unwrap_or_else(|| "24".into()).parse()?;
    let seed: u64 = args.next().unwrap_or_else(|| "54129".into()).parse()?;
    if replicas == 0 || replicas > 128 {
        return Err("replicate count must be between 1 and 128".into());
    }
    let revision = env::var("CL0_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut passed_replay = 0_u64;
    for index in 0..replicas {
        let trial_seed = seed.wrapping_add(index as u64);
        let world = seed_world(trial_seed);
        let cases = [
            ScheduledCondition { label: format!("seed={trial_seed}/control"), actions: vec![] },
            ScheduledCondition { label: format!("seed={trial_seed}/zero-credit"), actions: vec![
                ScheduledAction { after_attempts: 0,
                    action: Intervention::SetCredits { process: 1, credits: 0 } }
            ] },
            ScheduledCondition { label: format!("seed={trial_seed}/disable-after-8"), actions: vec![
                ScheduledAction { after_attempts: 8,
                    action: Intervention::Disable { process: 1 } }
            ] },
            ScheduledCondition { label: format!("seed={trial_seed}/environment-compute"), actions: vec![
                ScheduledAction { after_attempts: 0,
                    action: Intervention::SetComputePool { credits: 64 } }
            ] },
        ];
        for run in lab.run_scheduled(&world, &cases, 48, &revision)? {
            if !lab.verify_scheduled_replay(run.id)? {
                return Err(format!("replay mismatch in run {}", run.id).into());
            }
            passed_replay += 1;
            println!("seed={} condition={} attempts={} cells={} code={} hash={}",
                trial_seed, run.label, run.attempts, run.world.used_cells(),
                run.world.occupied_units(), run.final_sha256);
        }
    }
    println!("finished {} runs; all {} exact replays passed; registry={path}",
        passed_replay, passed_replay);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_seed_yields_identical_world_and_trace() {
        let mut a = seed_world(17);
        let mut b = seed_world(17);
        assert_eq!(a, b);
        a.run(48);
        b.run(48);
        assert_eq!(a, b);
    }
    #[test]
    fn seed_is_experimenter_variation_and_allocation_bounded() {
        let a = seed_world(17);
        let b = seed_world(18);
        assert_ne!(a, b);
        assert_eq!(a.occupied_units(), 13);
        assert_eq!(a.total_capacity(), Some(64));
        assert!(a.strict_accounting());
    }
}
