from typing import Callable, Optional
import core
import core.block
from core.block import Size, Block

Algorithm = Callable[[core.block.Size, list[core.block.Block]], Optional[int]]


def first_fit(size: Size, blocks: list[Block]) -> Optional[int]:
    for i, block in enumerate(blocks):
        if block.size >= size:
            return i
    return None


def best_fit(size: Size, blocks: list[Block]) -> Optional[int]:
    best_i: Optional[int] = None
    for i, block in enumerate(blocks):
        if block.size >= size:
            if not best_i or blocks[best_i].size > block.size:
                best_i = i
    return best_i


def worst_fit(size: Size, blocks: list[Block]) -> Optional[int]:
    worst_i: Optional[int] = None
    for i, block in enumerate(blocks):
        if block.size >= size:
            if not worst_i or blocks[worst_i].size < block.size:
                worst_i = i
    return worst_i
