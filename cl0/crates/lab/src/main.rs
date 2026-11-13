use cl0_lab::{Condition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "cl0-experiments.sqlite".into());
    // Explicitly constructed benchmark, not an instance of digital life.
    let mut initial = World::new(2, 16);
    initial
        .load(Process::new(
            1,
            vec![
                Op::Const {
                    dst: 0,
                    value: 7,
                    next: 1,
                },
                Op::Const {
                    dst: 1,
                    value: 42,
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
        .expect("valid externally constructed fixture");
    let source =
        std::env::var("CL0_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(db_path)?;
    let runs = lab.run_matched(
        &initial,
        &[
            Condition {
                label: "no-intervention".into(),
                interventions: vec![],
            },
            Condition {
                label: "disable-execution".into(),
                interventions: vec![Intervention::Disable { process: 1 }],
            },
            Condition {
                label: "no-compute-credit".into(),
                interventions: vec![Intervention::SetCredits {
                    process: 1,
                    credits: 0,
                }],
            },
        ],
        64,
        &source,
    )?;
    for run in runs {
        println!(
            "id={} label={} attempts={} key7={:?} replay={}",
            run.id,
            run.label,
            run.attempts,
            run.world.cell(7),
            lab.verify_replay(run.id)?
        );
    }
    Ok(())
}
