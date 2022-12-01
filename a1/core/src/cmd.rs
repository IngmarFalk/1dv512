use thiserror::Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unable to parse {0} as a command. [ Options are A (Allocate), D (Deallocate) and C (Compact) ].")]
    InvalidCommand(String),

    #[error("Unable to parse {0} as a block id. Block id must be a positive integer.")]
    InvalidBlockId(String),

    #[error("Unable to parse {0} as a block size. Block size must be a positive integer.")]
    InvalidBlockSize(String),

    #[error("Command {0} requires a block id and a block size.")]
    MissingParameters(String),

    #[error("Invalid Format: {0}")]
    Format(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CmdType {
    Alloc,
    Dealloc,
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
        Self::new(CmdType::Dealloc, Some(block_id), None)
    }

    pub fn compact() -> Self {
        Self::new(CmdType::Compact, None, None)
    }
}

impl TryFrom<String> for Cmd {
    type Error = ParseError;

    fn try_from(s: String) -> ParseResult<Self> {
        let mut iter = s.split(';');
        let cmd = iter.next().unwrap();
        match cmd {
            "A" => {
                let id = iter.next();
                let size = iter.next();
                match (id, size) {
                    (Some(id), Some(size)) => {
                        let id = id
                            .parse()
                            .map_err(|_| ParseError::InvalidBlockId(id.clone().to_owned()))?;
                        let size = size
                            .parse()
                            .map_err(|_| ParseError::InvalidBlockSize(size.clone().to_owned()))?;
                        Ok(Self::alloc(id, size))
                    }
                    _ => Err(ParseError::MissingParameters(cmd.clone().to_owned())),
                }
            }
            "D" => {
                let block_id = iter.next().unwrap().parse().unwrap();
                Ok(Self::dealloc(block_id))
            }
            "C" => Ok(Self::compact()),
            _ => Err(ParseError::InvalidCommand(cmd.clone().to_owned())),
        }
    }
}

impl TryFrom<&str> for Cmd {
    type Error = ParseError;

    fn try_from(s: &str) -> ParseResult<Self> {
        Self::try_from(s.to_owned())
    }
}
