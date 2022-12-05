from enum import Enum
from typing import Optional
from dataclasses import dataclass


class Op(Enum):
    Alloc = "A"
    Dealloc = "D"
    Compact = "C"


@dataclass
class Cmd:
    op: Op
    id: Optional[int]
    size: Optional[int]

    @staticmethod
    def alloc(id: int, size: int) -> "Cmd":
        return Cmd(Op.Alloc, id, size)

    @staticmethod
    def dealloc(id: int) -> "Cmd":
        return Cmd(Op.Dealloc, id, None)

    @staticmethod
    def compact() -> "Cmd":
        return Cmd(Op.Compact, None, None)

    @staticmethod
    def from_str(line: str) -> "Cmd":
        parts = line.split(";")
        if len(parts) == 1:
            return Cmd.compact()
        elif len(parts) == 3:
            return Cmd.alloc(int(parts[1]), int(parts[2]))
        elif len(parts) == 2:
            return Cmd.dealloc(int(parts[1]))
        else:
            raise ValueError("Invalid command")

    def get_id(self) -> int:
        if self.id is None:
            raise ValueError("Invalid command")
        return self.id

    def get_size(self) -> int:
        if self.size is None:
            raise ValueError("Invalid command")
        return self.size
