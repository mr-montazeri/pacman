use std::time::Duration;
use std::fmt;

#[macro_export]
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub struct DurationWrapper(pub Duration);

impl fmt::Display for DurationWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let micros = self.0.as_micros();
        if micros < 1000 {
            write!(f, "{} μs", micros)
        } else {
            let millis = micros as f32 / 1000f32;
            write!(f, "{:.2} ms", millis)
        }
    }
}