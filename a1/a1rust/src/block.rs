use std::{
    cmp::{max, min},
    ops::{Add, Sub},
};

macro_rules! impl_arith {
    ($for:ident, $other:ident) => {
        impl Add<$other> for $for {
            type Output = $for;
            fn add(self, rhs: $other) -> Self::Output {
                $for(self.0 + rhs.0)
            }
        }

        impl Sub<$other> for $for {
            type Output = $for;
            fn sub(self, rhs: $other) -> Self::Output {
                $for(self.0 - rhs.0)
            }
        }
    };
    ($for:ident, $other:ident, $($ty:ident),*) => {
        impl_arith!($for, $for);
        impl_arith!($for, $other);
        $(
            impl Add<$ty> for $for {
                type Output = $for;
                fn add(self, rhs: $ty) -> Self::Output {
                    $for(self.0 + rhs as usize)
                }
            }

            impl Sub<$ty> for $for {
                type Output = $for;
                fn sub(self, rhs: $ty) -> Self::Output {
                    $for(self.0 - rhs as usize)
                }
            }
        )*
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Id(pub usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Size(pub usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Address(pub usize);

impl_arith!(Address, Size, usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_arith!(Size, Address, usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub struct Block {
    pub id: Option<Id>,
    pub size: Size,
    pub start_addr: Address,
    pub end_addr: Address,
}

impl Block {
    pub fn new_free(size: Size, start_addr: Address) -> Self {
        let end_addr = start_addr + size - 1;
        Block {
            id: None,
            size,
            start_addr,
            end_addr,
        }
    }

    pub fn new_used(id: Id, size: Size, start_addr: Address) -> Self {
        let end_addr = start_addr + size - 1;
        Block {
            id: Some(id),
            size,
            start_addr,
            end_addr,
        }
    }

    pub fn is_free(&self) -> bool {
        self.id.is_none()
    }

    pub fn can_merge(&self, other: &Block) -> bool {
        self.is_free()
            && other.is_free()
            && (self.end_addr + 1 == other.start_addr || other.end_addr + 1 == self.start_addr)
    }

    pub fn merge(&self, other: &Block) -> Block {
        let s_addr = min(self.start_addr, other.start_addr);
        let e_addr = max(self.end_addr, other.end_addr);
        let size = Size(e_addr.0 - s_addr.0 + 1);
        Block::new_free(size, s_addr)
    }

    pub fn as_free(&self) -> Block {
        Block::new_free(Size(self.size.0), Address(self.start_addr.0))
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "{};{};{}", id.0, self.start_addr.0, self.end_addr.0),
            None => write!(f, "{};{}", self.start_addr.0, self.end_addr.0),
        }
    }
}

pub struct BlockVec(Vec<Block>);

impl BlockVec {
    pub fn new() -> BlockVec {
        BlockVec(vec![])
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, block: Block) {
        self.0.push(block)
    }

    pub fn pop(&mut self, i: usize) -> Block {
        self.0.remove(i)
    }

    pub fn get(&self, i: usize) -> &Block {
        &self.0[i]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_vec(&self) -> &Vec<Block> {
        &self.0
    }

    pub fn iter(&self) -> std::slice::Iter<Block> {
        self.0.iter()
    }
}
