import sys
import core
import core.cmd
import core.memory
import core.algorithms
import core.block
import copy
from typing import Callable, Optional


def read_file(path: str) -> list[str]:
    with open(path, "r") as f:
        return f.readlines()


def to_cmds(lines: list[str]) -> tuple[int, list[core.cmd.Cmd]]:
    size: int = int(lines[0])
    cmds: list[core.cmd.Cmd] = [core.cmd.Cmd.from_str(line) for line in lines[1:]]
    # cmds.reverse()
    return size, cmds


def did_occur(errors: dict[tuple[str,int], str], id: int) -> bool:
    maybe: Optional[str] = errors.get(("A", id))
    if maybe:
        return True
    return False

def main():
    # Read path from args
    path: str = sys.argv[1]
    data: list[str] = read_file(f"{path}.in")
    size, cmds = to_cmds(data)
    fns: dict[str, core.algorithms.Algorithm] = {
        "FirstFit": core.algorithms.first_fit,
        "BestFit": core.algorithms.best_fit,
        "WorstFit": core.algorithms.worst_fit,
    }

    
    for cmd in cmds:
        print(cmd)

    out: str = ""
    for name, fn in fns.items():
        instruction_cnt: int = 0
        errors: dict[tuple[str, int], str] = {}
        out_cnt: int = 0
        memory: core.memory.Memory = core.memory.Memory(core.block.Size(size))
        for c in cmds:
            instruction_cnt += 1
            res: core.memory.Result = core.memory.Result.Ok
            match c.op:
                case core.cmd.Op.Alloc:
                    res = memory.alloc(core.block.Id(c.get_id()), core.block.Size(c.get_size()), fn)
                case core.cmd.Op.Dealloc:
                    res = memory.dealloc(core.block.Id(c.get_id()))
                case core.cmd.Op.Compact:
                    memory.compact()
                case core.cmd.Op.Output:
                    cp: str = copy.deepcopy(out)
                    cp += f"{name}\n"
                    cp += str(memory)
                    cp += "Errors:\n" + "".join(errors.values()) + "\n" if len(errors) != 0 else "None\n"
                    with open(f"{path}.out{out_cnt}", "w") as f:
                        f.write(cp)
            match res:
                case core.memory.Result.Ok:
                    continue
                case core.memory.Result.Alloc:
                    errors[("A", c.get_id())] = f"A;{instruction_cnt};{sum([block.size for block in memory.free_blocks])}"
                case core.memory.Result.Dealloc:
                    reason: str = "1" if did_occur(errors, c.get_id()) else "0"
                    errors[("D", c.get_id())] = f"D;{instruction_cnt};{reason}"
            
        print(errors)

        out += f"{name}\n"
        out += str(memory)
        out += "Errors:\n" + "\n".join(errors.values()) + "\n\n" if len(errors) != 0 else "None\n"

    with open(f"{path}.out", "w") as f:
        f.write(out)

    for line in read_file(f"{path}.out"):
        print(line, end="")


if __name__ == "__main__":
    main()
