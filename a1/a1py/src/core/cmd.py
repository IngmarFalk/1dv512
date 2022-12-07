from enum import Enum
from typing import Optional
from dataclasses import dataclass


class Op(Enum):
    Alloc = "A"
    Dealloc = "D"
    Compact = "C"
    Output = "O"


@dataclass
class Cmd:
    op: Op
    id: Optional[int]
    size: Optional[int]

    def __init__(self, op: Op, id: Optional[int] = None, size: Optional[int] = None) -> None:
        self.op = op
        self.id = id
        self.size = size

    @staticmethod
    def from_str(line: str) -> "Cmd":
        parts: list[str] = [line[0]]
        if ";" in line:
            parts = line.split(";")
        match parts[0]:
            case "A":
                return Cmd(Op.Alloc, int(parts[1]), int(parts[2]))
            case "D":
                return Cmd(Op.Dealloc, int(parts[1]))
            case "C":
                return Cmd(Op.Compact)
            case "O":
                return Cmd(Op.Output)
            case _:
                raise ValueError("Invalid command")

    def get_id(self) -> int:
        if self.id is None:
            raise ValueError("Invalid command")
        return self.id

    def get_size(self) -> int:
        if self.size is None:
            raise ValueError("Invalid command")
        return self.size
