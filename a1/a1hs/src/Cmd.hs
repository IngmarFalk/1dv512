module Cmd (fromStr) where

data Cmd = Alloc Int Int | Dealloc Int | Compact | Print

fromStr :: String -> Int -> Int -> Cmd
fromStr "A" id size = Alloc id size
fromStr "D" id _ = Dealloc id
fromStr "C" _ _ = Compact
fromStr "O" _ _ = Print


