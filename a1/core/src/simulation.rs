use crate::{
    area::Area,
    cmd::{Cmd, CmdType, ParseError, ParseResult},
};

pub struct Simulation {
    pub area: Area,
    pub cmds: Vec<Cmd>,
}

impl Simulation {
    pub fn from_file(path: &str) -> ParseResult<Self> {
        let inp = std::fs::read_to_string(path)?;
        Self::from_str(&inp)
    }

    pub fn from_str(inp: &str) -> ParseResult<Self> {
        let itr = inp.split_whitespace().into_iter();
        let size = itr.clone().next();
        match size {
            Some(size) => {
                let size = size.parse()?;
                let area = Area::new(size);
                let mut cmds = Vec::new();
                for cmd in itr.skip(1) {
                    let cmd = Cmd::try_from(cmd)?;
                    cmds.push(cmd);
                }
                Ok(Self { area, cmds })
            }
            None => Err(ParseError::MissingParameters("size".to_owned())),
        }
    }

    pub fn run(&mut self) {
        for cmd in self.cmds.iter() {
            match cmd.ty {
                CmdType::Alloc => {
                    let block_id = cmd.block_id.unwrap();
                    let size = cmd.size.unwrap();
                    self.area.alloc_first_fit(block_id, size);
                }
                CmdType::Dealloc => {
                    let block_id = cmd.block_id.unwrap();
                    self.area.dealloc(block_id);
                }
                CmdType::Compact => {
                    self.area.compact();
                }
            }
        }
    }
}

mod simulation_tests {
    use crate::cmd::CmdType;

    #[test]
    fn test_from_file() {
        let sim = super::Simulation::from_file("../input/test.txt").unwrap();
        assert_eq!(sim.area.size, 1000);
        assert_eq!(sim.cmds.len(), 6);

        let cmd1 = &sim.cmds[0];
        assert_eq!(cmd1.block_id, Some(0));
        assert_eq!(cmd1.size, Some(100));
        assert_eq!(cmd1.ty, CmdType::Alloc);

        let cmd2 = &sim.cmds[1];
        assert_eq!(cmd2.block_id, Some(1));
        assert_eq!(cmd2.size, Some(100));
        assert_eq!(cmd2.ty, CmdType::Alloc);

        let cmd3 = &sim.cmds[2];
        assert_eq!(cmd3.block_id, Some(2));
        assert_eq!(cmd3.size, Some(500));
        assert_eq!(cmd3.ty, CmdType::Alloc);

        let cmd4 = &sim.cmds[3];
        assert_eq!(cmd4.block_id, Some(1));
        assert_eq!(cmd4.ty, CmdType::Dealloc);

        let cmd5 = &sim.cmds[4];
        assert_eq!(cmd5.block_id, Some(3));
        assert_eq!(cmd5.size, Some(200));
        assert_eq!(cmd5.ty, CmdType::Alloc);

        let cmd6 = &sim.cmds[5];
        assert_eq!(cmd6.block_id, Some(2));
        assert_eq!(cmd6.ty, CmdType::Dealloc);
    }

    #[test]
    fn test_from_str() {
        let input = r#"1000
A;0;100
A;1;100
A;2;500
D;1
A;3;200
D;2
        "#;

        let sim = super::Simulation::from_str(input).unwrap();
        assert_eq!(sim.area.size, 1000);
        assert_eq!(sim.cmds.len(), 6);

        let cmd1 = &sim.cmds[0];
        assert_eq!(cmd1.block_id, Some(0));
        assert_eq!(cmd1.size, Some(100));
        assert_eq!(cmd1.ty, CmdType::Alloc);

        let cmd2 = &sim.cmds[1];
        assert_eq!(cmd2.block_id, Some(1));
        assert_eq!(cmd2.size, Some(100));
        assert_eq!(cmd2.ty, CmdType::Alloc);

        let cmd3 = &sim.cmds[2];
        assert_eq!(cmd3.block_id, Some(2));
        assert_eq!(cmd3.size, Some(500));
        assert_eq!(cmd3.ty, CmdType::Alloc);

        let cmd4 = &sim.cmds[3];
        assert_eq!(cmd4.block_id, Some(1));
        assert_eq!(cmd4.ty, CmdType::Dealloc);

        let cmd5 = &sim.cmds[4];
        assert_eq!(cmd5.block_id, Some(3));
        assert_eq!(cmd5.size, Some(200));
        assert_eq!(cmd5.ty, CmdType::Alloc);

        let cmd6 = &sim.cmds[5];
        assert_eq!(cmd6.block_id, Some(2));
        assert_eq!(cmd6.ty, CmdType::Dealloc);
    }
}
