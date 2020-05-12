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
