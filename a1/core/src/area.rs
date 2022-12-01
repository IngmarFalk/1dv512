use crate::block::{Block, BlockId};
use thiserror::Error;

pub type AResult<T> = std::result::Result<T, AError>;

#[derive(Debug, Error)]
pub enum AError {
    #[error("Unable to allocate {0} bytes for block {1}. Available amount is {2} bytes.")]
    Alloc(u64, u64, u64),

    #[error("Unable to deallocate block {0}. Block does not exist.")]
    Dealloc(BlockId),

    #[error("Unable to compact. No free blocks available.")]
    Compact,

    #[error(transparent)]
    BlockError(#[from] crate::block::BError),
}

pub struct Area {
    pub free_blocks: Vec<Block>,
    pub used_blocks: Vec<Block>,
    pub size: u64,
}

impl Area {
    pub fn new(size: u64) -> Self {
        Self {
            size,
            free_blocks: vec![Block::new_free(0, size)],
            used_blocks: vec![],
        }
    }

    fn alloc<F: Fn(&mut Vec<Block>) -> Option<&mut Block>>(
        &mut self,
        // free_block: Option<&mut Block>,
        block_id: u64,
        size: u64,
        fun: F,
    ) -> AResult<()> {
        let free_block = fun(self.free_blocks.as_mut());
        match free_block {
            Some(block) => {
                let new_block = block.take(block_id, size);
                match new_block {
                    Ok(new_block) => {
                        self.used_blocks.push(new_block);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => Err(AError::Alloc(
                size,
                block_id,
                self.free_blocks.iter().map(|b| b.size).sum(),
            )),
        }
    }

    pub fn alloc_first_fit(&mut self, block_id: u64, size: u64) -> AResult<()> {
        self.alloc(block_id, size, |blocks| {
            blocks.iter_mut().find(|b| b.size >= size)
        })
    }

    pub fn alloc_best_fit(&mut self, block_id: u64, size: u64) -> AResult<()> {
        self.alloc(block_id, size, |blocks| {
            blocks.iter_mut().min_by_key(|b| b.size)
        })
    }

    pub fn alloc_worst_fit(&mut self, block_id: u64, size: u64) -> AResult<()> {
        self.alloc(block_id, size, |blocks| {
            blocks.iter_mut().max_by_key(|b| b.size)
        })
    }

    pub fn dealloc(&mut self, block_id: u64) -> AResult<()> {
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
                let mut mergable_block = None;
                for free_block in self.free_blocks.iter_mut() {
                    if free_block.can_merge(&block) {
                        mergable_block = Some(free_block);
                        break;
                    }
                }
                match mergable_block {
                    Some(b) => {
                        b.merge(block);
                        Ok(())
                    }
                    None => {
                        self.free_blocks.push(block);
                        Ok(())
                    }
                }
            }
            None => Err(AError::Dealloc(BlockId::Used(block_id))),
        }
    }

    pub fn compact(&mut self) -> AResult<()> {
        if self.free_blocks.is_empty() {
            return Err(AError::Compact);
        }

        self.free_blocks.sort_by(|a, b| a.cmp_start_addr(&b));
        self.used_blocks.sort_by(|a, b| a.cmp_start_addr(&b));

        let first_free_block = self.free_blocks.get(0).unwrap();
        let mut offset = 0;
        let mut new_used_blocks = vec![];

        for block in self.used_blocks.iter() {
            match block.start_addr > first_free_block.start_addr {
                true => new_used_blocks.push(Block::new(
                    block.id.clone(),
                    first_free_block.start_addr + offset,
                    block.size,
                )),
                false => {
                    new_used_blocks.push(Block::new(block.id.clone(), block.start_addr, block.size))
                }
            }
            offset += block.size;
        }

        let total_free_memory = self.calc_free_memory();
        let total_free_memory_block =
            Block::new_free(self.size - total_free_memory, total_free_memory);

        self.used_blocks = new_used_blocks;
        self.free_blocks = vec![total_free_memory_block];
        Ok(())
    }

    fn calc_free_memory(&self) -> u64 {
        self.free_blocks.iter().map(|b| b.size).sum()
    }
}

mod mem_tests {

    #[test]
    fn test_alloc_missing_memory() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 80).unwrap();
        assert_eq!(area.alloc_first_fit(3, 20).is_err(), true);
    }

    #[test]
    fn test_alloc() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 80).unwrap();
        assert_eq!(area.alloc_first_fit(3, 10).is_ok(), true);
    }

    #[test]
    fn test_alloc_memory_addresses() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 80).unwrap();
        area.alloc_first_fit(3, 10).unwrap();
        assert_eq!(area.used_blocks[0].start_addr, 0);
        assert_eq!(area.used_blocks[0].end_addr, 9);
        assert_eq!(area.used_blocks[1].start_addr, 10);
        assert_eq!(area.used_blocks[1].end_addr, 89);
        assert_eq!(area.used_blocks[2].start_addr, 90);
        assert_eq!(area.used_blocks[2].end_addr, 99);
    }

    #[test]
    fn test_compact() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 30).unwrap();
        area.alloc_first_fit(3, 20).unwrap();
        area.alloc_first_fit(4, 40).unwrap();

        // | 1: 10 |
        // | 2: 30 |
        // | 3: 20 |
        // | 4: 40 |

        area.dealloc(1).unwrap();
        area.dealloc(3).unwrap();

        // | _: 10 |
        // | 2: 30 |
        // | _: 20 |
        // | 4: 40 |

        area.compact().unwrap();

        // | 2: 30 |
        // | 4: 40 |
        // | _: 30 |

        assert_eq!(area.free_blocks.len(), 1);
        assert_eq!(area.used_blocks.len(), 2);
        assert_eq!(area.free_blocks[0].size, 30);
        assert_eq!(area.used_blocks[0].size, 30);
        assert_eq!(area.used_blocks[1].size, 40);
    }
}
