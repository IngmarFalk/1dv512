import sys
import core
import core.cmd
import core.memory
import core.algorithms
import core.block
from typing import Callable, Optional


def read_file(path: str) -> list[str]:
    with open(path, "r") as f:
        return f.readlines()


def to_cmds(lines: list[str]) -> tuple[int, list[core.cmd.Cmd]]:
    size: int = int(lines[0])
    cmd: list[core.cmd.Cmd] = [core.cmd.Cmd.from_str(line) for line in lines[1:]]
    return size, cmd


def main():
    # Read path from args
    path: str = sys.argv[1]
    data: list[str] = read_file(f"{path}.txt")
    size, cmds = to_cmds(data)
    fns: dict[str, core.algorithms.Algorithm] = {
        "FirstFit": core.algorithms.first_fit,
        "BestFit": core.algorithms.best_fit,
        "WorstFit": core.algorithms.worst_fit,
    }

    out: str = ""
    for name, fn in fns.items():
        memory: core.memory.Memory = core.memory.Memory(core.block.Size(size))
        for c in cmds:
            if c.op == core.cmd.Op.Alloc:
                memory.alloc(
                    core.block.Id(c.get_id()), core.block.Size(c.get_size()), fn
                )
            elif c.op == core.cmd.Op.Dealloc:
                memory.dealloc(core.block.Id(c.get_id()))
            elif c.op == core.cmd.Op.Compact:
                memory.compact()
        out += f"{name}\n"
        out += str(memory)

    with open(f"{path}_out.txt", "w") as f:
        f.write(out)

    for line in read_file(f"{path}_out.txt"):
        print(line, end="")


if __name__ == "__main__":
    main()
