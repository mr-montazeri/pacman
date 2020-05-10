use std::convert::TryFrom;
use std::io;

use crate::parse_input;
use crate::pac::{Pac, PacProperties};
use std::collections::HashMap;
use crate::pellet::Pellet;

#[repr(u8)]
#[derive(Debug)]
pub enum CellType {
    Wall = b'W',
    Floor = b' ',
}

impl TryFrom<u8> for CellType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CellType::Wall as u8 => Ok(CellType::Wall),
            x if x == CellType::Floor as u8 => Ok(CellType::Floor),
            _ => Err(()),
        }
    }
}

pub type Position = (usize, usize);

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Team {
    Us,
    Opp,
    Unknown,
}

impl Default for Team {
    fn default() -> Self { Team::Unknown }
}

pub type PacIdentifier = (Team, i32);

#[derive(Default)]
pub struct WorldModel {
    dim: (usize, usize),
    grid: Vec<CellType>,
    pacs: HashMap<PacIdentifier, Pac>,
    pellets: HashMap<Position, Pellet>,
    turn: u32,
    my_score: i32,
    opp_score: i32,
}

impl WorldModel {
    pub fn from_input() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let width = parse_input!(inputs[0], usize); // size of the grid
        let height = parse_input!(inputs[1], usize); // top left corner is (x=0, y=0)

        let mut cells: Vec<CellType> = Vec::new();

        for _ in 0..height {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim_end(); // one line of the grid: space " " is floor, pound "#" is wall
            for c in row.as_bytes() {
                cells.push(CellType::try_from(*c).unwrap());
            }
        }

        Self::new((width, height), cells)
    }

    pub fn new(dim: (usize, usize), grid: Vec<CellType>) -> Self {
        assert_eq!(dim.0 * dim.1, grid.len());
        WorldModel {
            dim,
            grid,
            ..Default::default()
        }
    }

    fn get_pac(&mut self, team: Team, id: i32) -> Option<&mut Pac> {
        let pac_key = (team, id);
        self.pacs.get_mut(&pac_key)
    }

    pub fn update_by_input(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_score = parse_input!(inputs[0], i32);
        let opp_score = parse_input!(inputs[1], i32);

        self.my_score = my_score;
        self.opp_score = opp_score;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pac_count = parse_input!(input_line, i32); // all your pacs and enemy pacs in sight

        for _ in 0..visible_pac_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // pac number (unique within a team)
            let mine = parse_input!(inputs[1], i32) == 1; // true if this pac is yours
            let x = parse_input!(inputs[2], usize); // position in the grid
            let y = parse_input!(inputs[3], usize); // position in the grid
            // let type_id = inputs[4].trim().to_string(); // unused in wood leagues
            // let speed_turns_left = parse_input!(inputs[5], i32); // unused in wood leagues
            // let ability_cooldown = parse_input!(inputs[6], i32); // unused in wood leagues
            let pos = (x, y);

            let team = if mine { Team::Us } else { Team::Opp };
            if let Some(pac) = self.get_pac(team, id) {
                pac.update(
                    PacProperties::new(pos)
                )
            } else {
                let pac_properties = PacProperties::new(pos);
                self.pacs.insert((team, id), Pac::new(team, id, Some(pac_properties)));
            }
        }
        // Since I have no idea what the mechanism for vision would be, I'll just remove all
        // pellets from World and recreate them. This might need changes later (e.g. confidence)

        self.pellets.clear();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pellet_count = parse_input!(input_line, i32); // all pellets in sight
        for _ in 0..visible_pellet_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], usize);
            let y = parse_input!(inputs[1], usize);
            let value = parse_input!(inputs[2], i32); // amount of points this pellet is worth
            let pos = (x, y);

            self.pellets.insert(pos, Pellet::new(pos, value));
        }
    }

    fn flat_index(&self, pos: Position) -> usize {
        assert!(pos.0 < self.dim.0);
        assert!(pos.1 < self.dim.1);

        let strides = (self.dim.1, 1);

        pos.0 * strides.0 + pos.1 * strides.1
    }

    pub fn kind(&self, pos: Position) -> &CellType {
        &self.grid[self.flat_index(pos)]
    }

    pub fn neighbors(&self, pos: Position) -> Vec<Position> {
        let mut out = Vec::new();

        let neighbors: [Position; 4] = [
            ((pos.0 - 1) % self.dim.0, pos.1), // Left
            (pos.0, (pos.1 - 1) % self.dim.1), // Top
            ((pos.0 + 1) % self.dim.0, pos.1), // Right
            (pos.0, (pos.1 + 1) % self.dim.1), // Bottom
        ];

        for neighbor in neighbors.iter() {
            let neighbor_kind = self.kind(*neighbor);
            match neighbor_kind {
                CellType::Floor => out.push(*neighbor),
                _ => (),
            }
        }

        out
    }

    pub fn get_pellets(&self) -> Vec<&Pellet> {
        self.pellets.values().collect()
    }

    pub fn get_team_pacs(&self) -> Vec<&Pac> {
        self.pacs.values().filter(|pac| pac.team() == Team::Us).collect()
    }

    pub fn pellet_at(&self, pos: Position) -> Option<&Pellet> {
        self.pellets.get(&pos)
    }
}