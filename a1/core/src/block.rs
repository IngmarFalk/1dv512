use std::ops::Add;

use thiserror::Error;

pub type BResult<T> = std::result::Result<T, BError>;

pub struct Addr(u64);

impl Into<u64> for Addr {
    fn into(self) -> u64 {
        self.0
    }
}

impl<V> Add<V> for Addr
where
    V: Into<u64>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        Self(self.0 + rhs.into())
    }
}

pub struct Size(u64);

impl Into<u64> for Size {
    fn into(self) -> u64 {
        self.0
    }
}

impl<V> Add<V> for Size
where
    V: Into<u64>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        Self(self.0 + rhs.into())
    }
}

#[derive(Debug, Error)]
pub enum BError {
    #[error(
        "Couldnt merge blocks {0} and {1} with sizes {2} and {3} and addresses ({4}, {5}) and ({6}, {7})"
    )]
    Merge(BlockId, BlockId, u64, u64, u64, u64, u64, u64),
    #[error("Couldnt take {0} bytes from block {1} with size {2} and address ({3}, {4})")]
    Take(u64, BlockId, u64, u64, u64),
}

impl BError {
    pub fn merge(b1: &Block, b2: &Block) -> Self {
        Self::Merge(
            b1.id.clone(),
            b2.id.clone(),
            b1.size,
            b2.size,
            b1.start_addr,
            b1.end_addr,
            b2.start_addr,
            b2.end_addr,
        )
    }

    pub fn take(b1: &Block, size: u64) -> Self {
        Self::Take(size, b1.id.clone(), b1.size, b1.start_addr, b1.end_addr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockId {
    Free,
    Used(u64),
}

impl PartialOrd for BlockId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (BlockId::Free, BlockId::Free) => Some(std::cmp::Ordering::Equal),
            (BlockId::Free, BlockId::Used(_)) => Some(std::cmp::Ordering::Less),
            (BlockId::Used(_), BlockId::Free) => Some(std::cmp::Ordering::Greater),
            (BlockId::Used(a), BlockId::Used(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for BlockId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockId::Free => write!(f, "Free"),
            BlockId::Used(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: BlockId,
    pub size: u64,
    pub start_addr: u64,
    pub end_addr: u64,
}

impl Block {
    pub fn new(id: BlockId, start_addr: u64, size: u64) -> Self {
        Self {
            id,
            size,
            start_addr,
            end_addr: start_addr + size - 1,
        }
    }

    pub fn new_used(id: u64, start_addr: u64, size: u64) -> Self {
        Self {
            id: BlockId::Used(id),
            size,
            start_addr,
            end_addr: start_addr + size - 1,
        }
    }

    pub fn new_free(start_addr: u64, size: u64) -> Self {
        Self {
            id: BlockId::Free,
            size,
            start_addr,
            end_addr: start_addr + size - 1,
        }
    }

    pub fn can_merge(&self, other: &Block) -> bool {
        other.start_addr == self.end_addr + 1
            || self.start_addr == other.end_addr + 1
                && self.id == BlockId::Free
                && other.id == BlockId::Free
    }

    pub fn merge(&self, other: Block) -> Block {
        let start_addr = self.start_addr.min(other.start_addr);
        let end_addr = self.end_addr.max(other.end_addr);

        // We have to add 1 to the size because the start_addr and end_addr are inclusive
        // Example: start_addr = 0, end_addr = 9, size = 10
        //          but 9 - 0 = 9, so we have to add 1 to get the correct size
        let size = end_addr - start_addr + 1;
        Block::new_free(start_addr, size)
    }

    pub fn merge_replace(&mut self, other: Block) -> BResult<()> {
        if self.can_merge(&other) {
            *self = self.merge(other);
            Ok(())
        } else {
            Err(BError::merge(self, &other))
        }
    }

    pub fn try_merge(&self, other: Block) -> BResult<Block> {
        if self.can_merge(&other) {
            Ok(self.merge(other))
        } else {
            Err(BError::merge(self, &other))
        }
    }

    pub fn take(&mut self, id: u64, size: u64) -> BResult<Block> {
        if !(self.id == BlockId::Free && self.size >= size) {
            return Err(BError::take(self, size));
        };

        let new_block = Block::new_used(id, self.start_addr, size);
        self.start_addr += size;
        self.size -= size;
        Ok(new_block)
    }

    pub fn as_free(&self) -> Block {
        Block::new_free(self.start_addr, self.size)
    }

    pub fn relocate(&mut self, new_start_addr: u64) {
        self.start_addr = new_start_addr;
        self.end_addr = new_start_addr + self.size - 1;
    }

    pub fn contains_addr(&self, addr: u64) -> bool {
        self.start_addr <= addr && addr <= self.end_addr
    }

    pub fn cmp_start_addr(&self, other: &Block) -> std::cmp::Ordering {
        self.start_addr.cmp(&other.start_addr)
    }

    pub fn cmp_end_addr(&self, other: &Block) -> std::cmp::Ordering {
        self.end_addr.cmp(&other.end_addr)
    }

    pub fn cmp_size(&self, other: &Block) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }

    pub fn cmp_id(&self, other: &Block) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

mod block_tests {

    #[test]
    fn test_free_creation() {
        let b1 = super::Block::new_free(0, 10);
        assert_eq!(b1.id, super::BlockId::Free);
        assert_eq!(b1.size, 10);
        assert_eq!(b1.start_addr, 0);
        assert_eq!(b1.end_addr, 9);
    }

    #[test]
    fn test_used_creation() {
        let b1 = super::Block::new_used(0, 0, 10);
        assert_eq!(b1.id, super::BlockId::Used(0));
        assert_eq!(b1.size, 10);
        assert_eq!(b1.start_addr, 0);
        assert_eq!(b1.end_addr, 9);
    }

    #[test]
    fn test_can_merge() {
        let b1 = super::Block::new_free(0, 100);
        let b2 = super::Block::new_free(100, 100);
        let b3 = super::Block::new_free(250, 100);

        assert_eq!(b1.can_merge(&b2), true);
        assert_eq!(b1.can_merge(&b3), false);
    }

    #[test]
    fn test_merging() {
        let b1 = super::Block::new_free(0, 100);
        let b2 = super::Block::new_free(100, 100);
        let b3 = b1.merge(b2);
        assert_eq!(b3.id, super::BlockId::Free);
        assert_eq!(b3.size, 200);
        assert_eq!(b3.start_addr, 0);
        assert_eq!(b3.end_addr, 199);
    }
}
