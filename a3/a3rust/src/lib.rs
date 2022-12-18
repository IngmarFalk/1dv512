use std::str::FromStr;

pub mod algorithms;
pub mod simulation;

#[derive(Debug, Clone)]
pub enum Direction {
    End,
    Start,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(0) => Ok(Self::End),
            Ok(1) => Ok(Self::Start),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(0) => Self::End,
            Ok(1) => Self::Start,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<usize> for Direction {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::End,
            1 => Self::Start,
            _ => panic!("Invalid direction"),
        }
    }
}
