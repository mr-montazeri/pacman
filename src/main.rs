mod agent;
mod world;
mod utils;

use crate::world::WorldModel;
use crate::agent::Agent;

/**
 * Grab the pellets as fast as you can!
 **/
fn main() {
    let mut agent = Agent::new(WorldModel::from_input());

    // game loop
    loop {
        agent.wm().update_by_input();
        eprintln!("=============== Step {}", agent.wm().turn());

        let result = agent.decide();

        println!("{}", result);
    }
}
