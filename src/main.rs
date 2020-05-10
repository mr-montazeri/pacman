mod world;
mod pac;
mod pellet;
mod utils;

use crate::world::{WorldModel, Position};
use std::collections::{HashSet, VecDeque};
use crate::pellet::Pellet;

fn nearest_pellet(wm: &WorldModel, start: Position) -> Option<&Pellet> {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while let Some(pos) = to_visit.pop_front() {
        let neighbors = wm.neighbors(pos);
        for cell in neighbors {
            if !visited.contains(&cell) {
                to_visit.push_back(cell);
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

        for pac in wm.get_team_pacs() {
            // Finding closest pellet
            match nearest_pellet(&wm, pac.pos()) {
                None => {
                    println!("MOVE {} {} {}", pac.id(), pac.pos().0, pac.pos().1);
                }
                Some(pellet) => {
                    println!("MOVE {} {} {}", pac.id(), pellet.pos().0, pellet.pos().1);
                }
            }
        }
    }
}
