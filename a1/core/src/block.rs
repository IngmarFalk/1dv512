use thiserror::Error;

pub type BResult<T> = std::result::Result<T, BError>;

#[derive(Debug, Error)]
pub enum BError {
    #[error(
        "Couldnt merge blocks {0} and {1} with sizes {2} and {3} and addresses ({4}, {5}) and ({6}, {7})"
    )]
    Merge(BlockId, BlockId, usize, usize, usize, usize, usize, usize),
    #[error("Couldnt take {0} bytes from block {1} with size {2} and address ({3}, {4})")]
    Take(usize, BlockId, usize, usize, usize),
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

    pub fn take(b1: &Block, size: usize) -> Self {
        Self::Take(size, b1.id.clone(), b1.size, b1.start_addr, b1.end_addr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockId {
    Free,
    Used(usize),
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockId::Free => write!(f, "Free"),
            BlockId::Used(id) => write!(f, "{}", id),
        }
    }
}

pub struct Block {
    pub id: BlockId,
    pub size: usize,
    pub start_addr: usize,
    pub end_addr: usize,
}

impl Block {
    pub fn new(id: usize, start_addr: usize, size: usize) -> Self {
        Self {
            id: BlockId::Used(id),
            size,
            start_addr,
            end_addr: start_addr + size - 1,
        }
    }

    pub fn new_free(start_addr: usize, size: usize) -> Self {
        Self {
            id: BlockId::Free,
            size,
            start_addr,
            end_addr: start_addr + size - 1,
        }
    }

    pub fn can_merge(&self, other: &Block) -> bool {
        self.end_addr == other.start_addr - 1
            || self.start_addr == other.end_addr - 1
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

    pub fn take(&mut self, size: usize) -> BResult<Block> {
        if !(self.id == BlockId::Free && self.size >= size) {
            return Err(BError::take(self, size));
        };

        let new_block = Block::new_free(self.start_addr, size);
        self.start_addr += size;
        self.size -= size;
        Ok(new_block)
    }

    pub fn as_free(&self) -> Block {
        Block::new_free(self.start_addr, self.size)
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
        let b1 = super::Block::new(0, 0, 10);
        assert_eq!(b1.id, super::BlockId::Used(0));
        assert_eq!(b1.size, 10);
        assert_eq!(b1.start_addr, 0);
        assert_eq!(b1.end_addr, 9);
    }

    #[test]
    fn test_can_merge() {
        let b1 = super::Block::new_free(0, 100);
        let b2 = super::Block::new_free(100, 100);
        assert_eq!(b1.can_merge(&b2), true);
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
