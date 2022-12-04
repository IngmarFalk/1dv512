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

#[derive(Debug, Clone)]
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

    fn alloc<F: Fn(Vec<Block>) -> Option<Block>>(
        &mut self,
        block_id: u64,
        size: u64,
        fun: F,
    ) -> AResult<()> {
        let free_block = fun(self.free_blocks.clone());
        match free_block {
            Some(block) => {
                let new_blocks = block.take(block_id, size);
                match new_blocks {
                    Ok(blocks) => {
                        self.take_from_free(&block, blocks);
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
            blocks.iter().find(|b| b.size >= size).cloned()
        })
    }

    pub fn alloc_best_fit(&mut self, block_id: u64, size: u64) -> AResult<()> {
        self.alloc(block_id, size, |blocks| {
            blocks.iter().min_by_key(|b| b.size).cloned()
        })
    }

    pub fn alloc_worst_fit(&mut self, block_id: u64, size: u64) -> AResult<()> {
        self.alloc(block_id, size, |blocks| {
            blocks.iter().max_by_key(|b| b.size).cloned()
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
                let mergable_block = self.free_blocks.iter_mut().find(|b| b.can_merge(&block));
                match mergable_block {
                    Some(b) => match b.merge_replace(block) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.into()),
                    },
                    None => Ok(self.free_blocks.push(block.as_free())),
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

        let _itr = self.used_blocks.iter();
        let _bs = _itr
            .map(|b| {
                offset += b.size;
                match b.start_addr >= first_free_block.start_addr {
                    true => Block::new(b.id.clone(), first_free_block.start_addr + offset, b.size),
                    false => Block::new(b.id.clone(), b.start_addr, b.size),
                }
            })
            .collect::<Vec<Block>>();

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

    fn take_from_free(&mut self, free_block: &Block, new_blocks: (Block, Block)) {
        let idx = self
            .free_blocks
            .iter()
            .position(|b| b == free_block)
            .unwrap();
        self.free_blocks.remove(idx);

        self.used_blocks.push(new_blocks.0);
        if new_blocks.1.size > 0 {
            self.free_blocks.push(new_blocks.1);
        }
    }

    pub fn as_byte_array(&self) -> Vec<u8> {
        let mut bytes = vec![0; self.size as usize];
        let mut blocks: Vec<&Block> = self
            .used_blocks
            .iter()
            .chain(self.free_blocks.iter())
            .collect();
        blocks.sort_by(|a, b| a.cmp_start_addr(&b));
        for block in blocks {
            for i in block.start_addr..=block.end_addr {
                if let BlockId::Used(_) = block.id {
                    bytes[i as usize] = 1
                }
            }
        }
        bytes
    }

    pub fn fragmentation(&self) -> f64 {
        let largest_block = self.free_blocks.iter().max_by_key(|b| b.size).unwrap().size;
        let free_memory = self.calc_free_memory();
        let fragmentation = 1f64 - (largest_block as f64 / free_memory as f64);
        fragmentation
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
    fn test_dealloc() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 40).unwrap();
        area.dealloc(1).unwrap();
        assert_eq!(area.free_blocks.len(), 2);
        area.alloc_first_fit(3, 40).unwrap();
        area.dealloc(2).unwrap();
        assert_eq!(area.free_blocks.len(), 2);
        assert_eq!(area.used_blocks.len(), 1);
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

    #[test]
    fn test_byte_array() {
        let mut area = super::Area::new(100);
        area.alloc_first_fit(1, 10).unwrap();
        area.alloc_first_fit(2, 30).unwrap();
        area.alloc_first_fit(3, 20).unwrap();
        area.alloc_first_fit(4, 40).unwrap();

        area.dealloc(1).unwrap();
        area.dealloc(3).unwrap();

        let bytes = area.as_byte_array();
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[9], 0);
        assert_eq!(bytes[10], 1);
        assert_eq!(bytes[39], 1);
        assert_eq!(bytes[40], 0);
        assert_eq!(bytes[59], 0);
        assert_eq!(bytes[60], 1);
        assert_eq!(bytes[99], 1);
    }

    #[test]
    fn test_example_scenario() {
        let mut area = super::Area::new(1000);
        area.alloc_first_fit(0, 100).unwrap();
        area.alloc_first_fit(1, 100).unwrap();
        area.alloc_first_fit(2, 500).unwrap();
        area.dealloc(1).unwrap();
        area.alloc_first_fit(3, 200).unwrap();
        area.dealloc(2).unwrap();
        assert_eq!(area.fragmentation(), 0.1428571428571429);
        assert_eq!(area.free_blocks.len(), 2);
        assert_eq!(area.used_blocks.len(), 2);
    }
}
