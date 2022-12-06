use super::block::{Block, Size};

pub enum AlgoResult {
    Ok(usize),
    None,
}

pub type Algo = for<'a> fn(Size, &'a Vec<Block>) -> AlgoResult;

pub fn first_fit(size: Size, blocks: &Vec<Block>) -> AlgoResult {
    for (i, block) in blocks.iter().enumerate() {
        if block.size >= size {
            return AlgoResult::Ok(i);
        }
    }
    AlgoResult::None
}

pub fn best_fit(size: Size, blocks: &Vec<Block>) -> AlgoResult {
    let mut best_block: AlgoResult = AlgoResult::None;
    for (i, block) in blocks.iter().enumerate() {
        if block.size >= size {
            match best_block {
                AlgoResult::Ok(best) => {
                    if block.size < blocks[best.clone()].size {
                        best_block = AlgoResult::Ok(i);
                    }
                }
                AlgoResult::None => {
                    best_block = AlgoResult::Ok(i);
                }
            }
        }
    }
    best_block
}

pub fn worst_fit(size: Size, blocks: &Vec<Block>) -> AlgoResult {
    let mut worst_block: AlgoResult = AlgoResult::None;
    for (i, block) in blocks.iter().enumerate() {
        if block.size >= size {
            match worst_block {
                AlgoResult::Ok(worst) => {
                    if block.size > blocks[worst.clone()].size {
                        worst_block = AlgoResult::Ok(i);
                    }
                }
                AlgoResult::None => {
                    worst_block = AlgoResult::Ok(i);
                }
            }
        }
    }
    worst_block
}
