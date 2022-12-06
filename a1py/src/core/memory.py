from dataclasses import dataclass
from typing import Callable, Optional
import core
import core.algorithms
import enum
from core.block import Block, Size, Address, Id


class Result(enum.Enum):
    Ok = "Ok"
    Alloc = "Alloc"
    Dealloc = "Dealloc"


@dataclass
class Memory:
    size: int
    free_blocks: list[Block]
    used_blocks: list[Block]

    def __init__(self, size: Size) -> None:
        self.size = size
        self.free_blocks = [Block.free(size, Address(0))]
        self.used_blocks = []

    def __str__(self) -> str:
        u_blocks: list[Block] = self.used_blocks
        u_blocks.sort(key=lambda block: block.id)
        f_blocks: list[Block] = self.free_blocks
        f_blocks.sort(key=lambda block: block.start_addr)
        used: str = "".join([str(block) for block in u_blocks])
        free: str = "".join([str(block) for block in f_blocks])
        return f"Size: {self.size}\nUsed:\n{used}Free:\n{free}Fragmentation:\n{self.fragmentation():.6f}\n"

    def alloc(self, id: Id, size: Size, fn: core.algorithms.Algorithm) -> Result:
        block_i: Optional[int] = fn(size, self.free_blocks)
        if block_i is None:
            return Result.Alloc

        block: Block = self.free_blocks[block_i]
        self.free_blocks.remove(block)
        self.used_blocks.append(Block.used(id, size, block.start_addr))
        if block.size > size:
            self.free_blocks.append(
                block.free(Size(block.size - size), Address(block.start_addr + size))
            )
        return Result.Ok

    def dealloc(self, id: Id) -> Result:
        for block in self.used_blocks:
            if block.id == id:
                for i, free_block in enumerate(self.free_blocks):
                    if free_block.can_merge(block.as_free()):
                        self.free_blocks[i] = free_block.merge(block.as_free())
                        self.used_blocks.remove(block)
                        return Result.Ok
                self.used_blocks.remove(block)
                self.free_blocks.append(block.as_free())
                return Result.Ok
        return Result.Dealloc

    def compact(self) -> None:

        blocks: list[Block] = self.used_blocks + self.free_blocks
        blocks.sort(key=lambda block: block.start_addr)

        ff_block: Optional[Block] = None
        front_offest: int = 0
        if len(self.free_blocks) > 0:
            ff_block = self.free_blocks[0]
            front_offest = ff_block.start_addr

        lf_block: Optional[Block] = None
        back_offest: int = 0
        if len(self.free_blocks) > 1:
            lf_block = self.free_blocks[-1]
            back_offest = self.size - (lf_block.start_addr + lf_block.size)

        for _, block in enumerate(blocks):
            if block.is_free():
                self.free_blocks.remove(block)
                self.free_blocks.append(
                    block.free(Size(block.size), Address(back_offest))
                )
                back_offest += block.size
            else:
                self.used_blocks.remove(block)
                self.used_blocks.append(
                    block.used(block.id, Size(block.size), Address(front_offest))
                )
                front_offest += block.size

    def fragmentation(self) -> float:
        self.free_blocks.sort(key=lambda block: block.size)
        return 1 - (
            self.free_blocks[-1].size / sum([block.size for block in self.free_blocks])
        )
