use crate::{algorithms::Algorithm, Direction};
use std::str::FromStr;

#[derive(Debug)]
pub struct Simulation {
    algorithm: Algorithm,
    data: Data,
}

impl Simulation {
    pub fn run(&self) -> (usize, Vec<usize>) {
        (self.algorithm)(
            self.data.cylinders,
            self.data.head,
            self.data.requests.clone(),
            self.data.dir.clone(),
        )
    }
}

#[derive(Debug)]
pub struct Data {
    cylinders: usize,
    head: usize,
    dir: Direction,
    requests: Vec<usize>,
}

impl Data {
    pub fn new(nrc: usize, head: usize, cylinders: Vec<usize>, dir: Option<Direction>) -> Self {
        Self {
            cylinders: nrc,
            head,
            dir: dir.unwrap_or(Direction::End),
            requests: cylinders,
        }
    }
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let nrc = lines
            .next()
            .ok_or("No number of cylinders")?
            .parse()
            .map_err(|e| format!("Invalid number of cylinders: {}", e))?;
        let head = lines
            .next()
            .ok_or("No head position")?
            .parse()
            .map_err(|e| format!("Invalid head position: {}", e))?;
        let dir = lines
            .next()
            .ok_or("No direction")?
            .parse()
            .map_err(|e| format!("Invalid direction: {}", e))?;
        let cylinders = lines
            .map(|line| line.parse().map_err(|e| format!("Invalid cylinder: {}", e)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(nrc, head, cylinders, Some(dir)))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
