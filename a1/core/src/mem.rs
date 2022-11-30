use crate::block::{Block, BlockId};
use thiserror::Error;

pub type MResult<T> = std::result::Result<T, MError>;

#[derive(Debug, Error)]
pub enum MError {
    #[error("Unable to allocate {0} bytes for block {1}. Available amount is {2} bytes.")]
    Alloc(usize, usize, usize),

    #[error("Unable to deallocate block {0}. Block does not exist.")]
    Dealloc(usize),

    #[error("Unable to compact. No free blocks available.")]
    Compact,

    #[error(transparent)]
    BlockError(#[from] crate::block::BError),
}

pub struct Memory {
    pub free_blocks: Vec<Block>,
    pub used_blocks: Vec<Block>,
    pub size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            free_blocks: vec![Block::new_free(0, size)],
            used_blocks: vec![],
        }
    }

    pub fn alloc(&mut self, block_id: usize, size: usize) -> MResult<()> {
        let mut free_block = None;
        for block in self.free_blocks.iter_mut() {
            if block.size >= size {
                free_block = Some(block);
                break;
            }
        }

        match free_block {
            Some(block) => {
                let new_block = block.take(size);
                match new_block {
                    Ok(new_block) => {
                        self.used_blocks.push(new_block);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => Err(MError::Alloc(
                size,
                block_id,
                self.free_blocks.iter().map(|b| b.size).sum(),
            )),
        }
    }

    pub fn dealloc(&mut self, block_id: usize) -> MResult<()> {
        let mut used_block_idx = None;
        for (i, block) in self.used_blocks.iter().enumerate() {
            if block.id == BlockId::Used(block_id) {
                used_block_idx = Some(i);
                break;
            }
        }

        match used_block_idx {
            Some(i) => {
                let block = self.used_blocks.remove(i);
                for free_block in self.free_blocks.iter_mut() {
                    if free_block.can_merge(&block) {
                        match free_block.merge_replace(block) {
                            Ok(_) => return Ok(()),
                            Err(e) => return Err(e.into()),
                        }
                    }
                }
                self.free_blocks.push(block.as_free());
                Ok(())
            }
            None => Err(MError::Dealloc(block_id)),
        }
    }
}

mod mem_tests {

    #[test]
    fn test_alloc() {
        let mut mem = super::Memory::new(100);
        mem.alloc(1, 10).unwrap();
        mem.alloc(2, 80).unwrap();
        assert_eq!(mem.alloc(3, 20).is_err(), true);
    }
}
