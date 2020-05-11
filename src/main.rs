mod world;
mod pac;
mod pellet;
mod utils;

use crate::world::{WorldModel, Position, DurationWrapper};
use std::collections::{HashSet, VecDeque};
use crate::pellet::Pellet;
use std::fmt::{self, Display, Formatter};

fn nearest_pellet(wm: &WorldModel, start: Position) -> Option<&Pellet> {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while let Some(pos) = to_visit.pop_front() {
        eprintln!("At {:?}", pos);
        visited.insert(pos);
        let neighbors = wm.neighbors(pos);
        eprintln!("Neighbors: {:?}", neighbors);
        for cell in neighbors {
            if !visited.contains(&cell) {
                eprintln!("Will visit: {:?}", cell);
                to_visit.push_back(cell);
            } else {
                eprintln!("Already visited: {:?}", cell);
            }
        }
        if let Some(pellet) = wm.pellet_at(pos) {
            return Some(pellet);
        }
    }
    None
}

/**
 * Grab the pellets as fast as you can!
 **/
fn main() {
    let mut wm = WorldModel::from_input();

    // game loop
    loop {
        wm.update_by_input();
        eprintln!("=============== Step {}", wm.turn());
        let start = wm.turn_start();

        let mut result = Vec::new();

        for pac in wm.get_team_pacs() {
            // Finding closest pellet
            eprintln!("{:?} => Pac {} at {:?}", start.elapsed(), pac.id(), pac.pos());

            let target = nearest_pellet(&wm, pac.pos());


            match target {
                None => {
                    eprintln!("Cant find pellet :(");
                    result.push(format!("MOVE {} {} {} {}", pac.id(), pac.pos().0, pac.pos().1, DurationWrapper(start.elapsed())))
                }
                Some(pellet) => {
                    eprintln!("Going to {:?}", pellet.pos());
                    result.push(format!("MOVE {} {} {} {}", pac.id(), pellet.pos().0, pellet.pos().1, DurationWrapper(start.elapsed())));
                }
            }
        }

        eprintln!("{:?} => Done!", start.elapsed());
        let result = result.join(" | ");
        println!("{}", result);
    }
}
