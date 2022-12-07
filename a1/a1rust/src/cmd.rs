use std::str::{FromStr, Split};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cmd {
    Alloc(usize, usize),
    Dealloc(usize),
    Compact,
    Output,
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cmd::Alloc(_, _) => write!(f, "A"),
            Cmd::Dealloc(_) => write!(f, "D"),
            Cmd::Compact => write!(f, "C"),
            Cmd::Output => write!(f, "O"),
        }
    }
}

impl FromStr for Cmd {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split(';');
        let (cmd, iter) = next_string(iter);
        match cmd.as_str() {
            "A" => {
                let (size, iter) = next_usize(iter);
                let (align, _) = next_usize(iter);
                Ok(Cmd::Alloc(size, align))
            }
            "D" => {
                let (id, _) = next_usize(iter);
                Ok(Cmd::Dealloc(id))
            }
            "O" => Ok(Cmd::Output),
            _ => Ok(Cmd::Compact),
        }
    }
}

fn next_string(mut iter: Split<char>) -> (String, Split<char>) {
    (iter.next().unwrap().to_owned(), iter)
}

fn next_usize(mut iter: Split<char>) -> (usize, Split<char>) {
    (iter.next().unwrap().parse().unwrap(), iter)
}

#[derive(Debug)]
pub struct CmdVec {
    pub size: usize,
    pub cmds: Vec<Cmd>,
}

impl CmdVec {
    pub fn new(size: usize) -> CmdVec {
        CmdVec { size, cmds: vec![] }
    }

    pub fn add(&mut self, cmd: Cmd) {
        self.cmds.push(cmd);
    }

    pub fn iter(&self) -> std::slice::Iter<Cmd> {
        self.cmds.iter()
    }
}

impl FromStr for CmdVec {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let size = lines.first().unwrap().parse().unwrap();
        let mut cmds = CmdVec::new(size);
        for line in lines[1..].iter() {
            let cmd = line.parse::<Cmd>()?;
            cmds.add(cmd);
        }
        Ok(cmds)
    }
}
