module Block (
    Size,
    Address,
    Id,
    Block(..),
    )   where

data Id a = Free | Used a 
data Size s = Size s 
data Address a = Address a 


data Block a s = Block (Size s) (Address a) 
