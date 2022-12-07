from dataclasses import dataclass
from enum import Enum
from typing import NewType

Id = NewType("Id", int)
Free = Id(-1)
Used = NewType("UsedId", Id)
Address = NewType("Address", int)
Size = NewType("Size", int)


@dataclass
class Block:
    id: Id
    size: Size
    start_addr: Address
    end_addr: Address

    @staticmethod
    def free(size: Size, start_addr: Address) -> "Block":
        return Block(Free, size, start_addr, Address(start_addr + size - 1))

    @staticmethod
    def used(id: Id, size: Size, start_addr: Address) -> "Block":
        return Block(id, size, start_addr, Address(start_addr + size - 1))

    def is_free(self) -> bool:
        return self.id == Free

    def __str__(self) -> str:
        if self.is_free():
            return f"{self.start_addr};{self.end_addr}\n"
        return f"{self.id};{self.start_addr};{self.end_addr}\n"

    def as_free(self) -> "Block":
        if self.is_free():
            return self
        return Block.free(self.size, self.start_addr)

    def can_merge(self, other: "Block") -> bool:
        return (
            self.is_free()
            and other.is_free()
            and (
                self.start_addr == other.end_addr + 1
                or other.start_addr == self.end_addr + 1
            )
        )

    def merge(self, other: "Block") -> "Block":
        if not self.can_merge(other):
            raise ValueError("Cannot merge blocks")
        start_addr: int = min(self.start_addr, other.start_addr)
        end_addr: int = max(self.end_addr, other.end_addr)
        return Block.free(Size(end_addr - start_addr + 1), start_addr)
