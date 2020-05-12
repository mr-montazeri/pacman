pub mod pac_kind;

use crate::world::{Position, Team};
use crate::world::pac::pac_kind::PacKind;

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
