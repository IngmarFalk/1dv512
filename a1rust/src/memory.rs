use std::io::Write;

use crate::{
    algos::{Algo, AlgoResult},
    block::{Address, Block, BlockVec, Id, Size},
    cmd::Cmd,
};

pub enum Result {
    Ok,
    AllocErr(Id, usize, usize),
    DeallocErr(Id, usize, usize),
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Ok => write!(f, "OK"),
            Result::AllocErr(_, instr_nr, size) => write!(f, "A;{};{}", instr_nr, size),
            Result::DeallocErr(_, instr_nr, reason) => write!(f, "D;{};{}", instr_nr, reason),
        }
    }
}

pub struct Memory {
    pub size: usize,
    pub free_blocks: BlockVec,
    pub used_blocks: BlockVec,
    errors: Vec<Result>,
    instr_cnt: usize,
    out_cnt: usize,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let mut free_blocks = BlockVec::new();
        free_blocks.add(Block::new_free(Size(size), Address(0)));
        Memory {
            size,
            free_blocks,
            used_blocks: BlockVec::new(),
            errors: vec![],
            instr_cnt: 0,
            out_cnt: 0,
        }
    }

    pub fn exec(&mut self, cmd: &Cmd, algo: Algo, path: &str) {
        self.incr();
        let res = match cmd {
            Cmd::Alloc(id, size) => self.alloc(Id(id.clone()), Size(size.clone()), algo),
            Cmd::Dealloc(id) => self.dealloc(Id(id.clone())),
            Cmd::Compact => self.compact(),
            Cmd::Output => {
                let out = self.output();
                let mut file =
                    std::fs::File::create(format!("{}.out{}", path, self.out_cnt)).unwrap();
                file.write_all(out.as_bytes()).unwrap();
                self.out_cnt += 1;
                Result::Ok
            }
        };

        match res {
            Result::Ok => (),
            _ => self.errors.push(res),
        }
    }

    pub fn free_memory(&self) -> usize {
        self.free_blocks.iter().fold(0, |acc, b| acc + b.size.0)
    }

    fn alloc(&mut self, id: Id, size: Size, func: fn(Size, &Vec<Block>) -> AlgoResult) -> Result {
        let index = func(size, &self.free_blocks.as_vec());
        match index {
            AlgoResult::Ok(i) => {
                let mut block = self.free_blocks.pop(i);
                let new_block = Block::new_used(id, size, block.start_addr);
                if new_block.size < block.size {
                    block.start_addr = block.start_addr + size;
                    block.size = block.size - size;
                    self.free_blocks.add(block);
                }
                self.used_blocks.add(new_block);
                Result::Ok
            }
            AlgoResult::None => Result::AllocErr(id, self.instr_cnt, self.free_memory()),
        }
    }

    fn dealloc(&mut self, id: Id) -> Result {
        for (i, b) in self.used_blocks.iter().enumerate() {
            if b.id.unwrap() == id {
                let mut block = self.used_blocks.pop(i);
                block.id = None;
                for (i, b) in self.free_blocks.iter().enumerate() {
                    if block.can_merge(b) {
                        let new_block = block.merge(b);
                        self.free_blocks.pop(i);
                        self.free_blocks.add(new_block);
                        return Result::Ok;
                    }
                }
                self.free_blocks.add(block);
                return Result::Ok;
            }
        }
        Result::DeallocErr(id, self.instr_cnt, self.did_try_allocating(id))
    }

    fn did_try_allocating(&self, id: Id) -> usize {
        let did_try: Option<&Result> = self.errors.iter().find(|e| match e {
            Result::AllocErr(i, _, _) => i == &id,
            _ => false,
        });

        match did_try {
            Some(Result::AllocErr(_, _, _)) => 1,
            _ => 0,
        }
    }

    fn compact(&mut self) -> Result {
        let mut nf_blocks = BlockVec::new();
        let mut nu_blocks = BlockVec::new();
        let f_mem = self.free_memory();

        let mut last_start_addr = Address(0);
        for block in self.used_blocks.iter() {
            let new_block =
                Block::new_used(block.id.unwrap(), block.size, last_start_addr + block.size);
            last_start_addr = new_block.end_addr + 1;
            nu_blocks.add(new_block);
        }
        nf_blocks.add(Block::new_free(Size(f_mem), Address(self.size - f_mem)));

        self.free_blocks = nf_blocks;
        self.used_blocks = nu_blocks;
        Result::Ok
    }

    pub fn output(&self) -> String {
        let mut out = format!("Size:\n{}\n", self.size);

        out.push_str("Allocated blocks:\n");
        for ub in self.used_blocks.iter() {
            out.push_str(&format!("{}\n", ub));
        }

        out.push_str("Free blocks:\n");
        for fb in self.free_blocks.iter() {
            out.push_str(&format!("{}\n", fb));
        }

        out.push_str(&format!("Fragmentation:\n{}\n", self.fragmentation()));

        out.push_str("Errors:\n");
        for err in self.errors.iter() {
            out.push_str(&format!("{}\n", err));
        }
        if self.errors.is_empty() {
            out.push_str("None\n");
        }
        out += "\n";

        out
    }

    fn fragmentation(&self) -> f64 {
        let largest_block = self.free_blocks.iter().max_by_key(|b| b.size).unwrap().size;
        let free_memory = self.free_memory();
        1f64 - largest_block.0 as f64 / free_memory as f64
    }

    fn incr(&mut self) {
        self.instr_cnt += 1;
    }
}
