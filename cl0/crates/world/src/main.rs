use cl0_world::{Op, Process, World};

fn main() {
    // This is an externally constructed execution fixture, NOT a digital organism.
    let mut world = World::new(2, 16);
    world
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
            8,
        ))
        .expect("valid fixture");
    world.run(32);
    for event in world.events() {
        println!("{event:?}");
    }
    println!(
        "cell[7]={:?}, credits_left={}",
        world.cell(7),
        world.remaining_credits()
    );
}
