module Main (main) where

import Lib
import Memory
import Block

m :: Memory Int [Block] [Block]
m = newMemory 100

main :: IO ()
main = do
    print m