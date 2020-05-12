use std::collections::{HashSet, VecDeque};

use crate::utils;
use crate::world::{WorldModel, Position};
use crate::world::pellet::Pellet;

pub struct Agent {
    wm: WorldModel,
}


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


impl Agent {
    pub fn new(wm: WorldModel) -> Self {
        Self {
            wm
        }
    }

    pub fn decide(&mut self) -> String {
        let mut result = Vec::new();

        let start = self.wm.turn_start();

        for pac in self.wm.get_team_pacs() {
            eprintln!("{:?} => Pac {} at {:?}", start.elapsed(), pac.id(), pac.pos());

            // Finding closest pellet
            let target = nearest_pellet(&self.wm, pac.pos());

            let message = utils::DurationWrapper(start.elapsed());

            match target {
                None => {
                    eprintln!("Cant find pellet :(");
                    result.push(format!("MOVE {} {} {} {}", pac.id(), pac.pos().0, pac.pos().1, message))
                }
                Some(pellet) => {
                    eprintln!("Going to {:?}", pellet.pos());
                    result.push(format!("MOVE {} {} {} {}", pac.id(), pellet.pos().0, pellet.pos().1, message));
                }
            }
        }

        eprintln!("{:?} => Done!", start.elapsed());
        result.join(" | ")
    }
    pub fn wm(&mut self) -> &mut WorldModel { &mut self.wm }
}