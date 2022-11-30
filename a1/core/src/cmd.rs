pub enum CmdType {
    Alloc,
    Dealloc(u64),
    Compact,
}

pub struct Cmd {
    pub ty: CmdType,
    pub block_id: Option<u64>,
    pub size: Option<u64>,
}

impl Cmd {
    pub fn new(ty: CmdType, block_id: Option<u64>, size: Option<u64>) -> Self {
        Self { ty, block_id, size }
    }

    pub fn alloc(block_id: u64, size: u64) -> Self {
        Self::new(CmdType::Alloc, Some(block_id), Some(size))
    }

    pub fn dealloc(block_id: u64) -> Self {
        Self::new(CmdType::Dealloc(block_id), None, None)
    }

    pub fn compact() -> Self {
        Self::new(CmdType::Compact, None, None)
    }
}

impl From<String> for Cmd {
    fn from(s: String) -> Self {
        let mut iter = s.split(';');
        let cmd = iter.next().unwrap();
        match cmd {
            "A" => {
                let id = iter.next().unwrap().parse().unwrap();
                let size = iter.next().unwrap().parse().unwrap();
                Self::alloc(id, size)
            }
            "D" => {
                let block_id = iter.next().unwrap().parse().unwrap();
                Self::dealloc(block_id)
            }
            "C" => Self::compact(),
            _ => panic!("invalid command"),
        }
    }
}
