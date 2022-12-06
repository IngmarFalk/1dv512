module Memory (
    Memory(..),
    )   where

import Block
import Data.List

data BlockList elems = BlockList [elems]
data Memory s f u = Memory (Size s) (BlockList f) (BlockList u)

newMemory :: Int -> Memory Int List List
newMemory size = Memory (Size size) (BlockList []) (BlockList [])

fragmentation :: Int -> Int -> Float
fragmentation largest free = 1 - (largest / free)
