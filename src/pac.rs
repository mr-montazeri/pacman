use crate::world::{Position, Team};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum PacKind {
    Rock,
    Paper,
    Scissors,
    Uninitialized,
}

impl fmt::Display for PacKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for PacKind {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            x if x == "ROCK" => Ok(PacKind::Rock),
            x if x == "PAPER" => Ok(PacKind::Paper),
            x if x == "SCISSORS" => Ok(PacKind::Scissors),
            _ => Err(()),
        }
    }
}

impl Default for PacKind {
    fn default() -> Self {
        Self::Uninitialized
    }
}

#[derive(Clone, Copy, Default)]
pub struct PacProperties {
    // properties
    kind: PacKind,
    pos: Position,
    speed_turns_left: i32,
    ability_cooldown: i32,
}

impl PacProperties {
    pub fn new(
        kind: PacKind,
        pos: Position,
        speed_turns_left: i32,
        ability_cooldown: i32,
    ) -> PacProperties {
        PacProperties {
            kind,
            pos,
            speed_turns_left,
            ability_cooldown,
        }
    }
}

#[derive(Default)]
pub struct Pac {
    // identifiers
    team: Team,
    id: i32,

    // properties
    prop: PacProperties,
}

impl Pac {
    pub fn new(team: Team, id: i32, prop: Option<PacProperties>) -> Self {
        if let Some(prop) = prop {
            Self { team, id, prop }
        } else {
            Self { team, id, ..Default::default() }
        }
    }

    pub fn update(&mut self, prop: PacProperties) {
        self.prop = prop;
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn team(&self) -> Team {
        self.team
    }
    pub fn pos(&self) -> Position {
        self.prop.pos
    }
    pub fn kind(&self) -> PacKind { self.prop.kind }
}
