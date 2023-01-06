"""The solution to day 5."""
import typing
import re, collections

from .aoc_helper import Solution

Instruction = typing.NamedTuple("Instruction", [("count", int), ("source", int), ("dest", int)])


class Day(Solution):
    """The solution to day 5."""

    day = 5

    def __init__(self, raw: str):
        self.stacks: dict[int, list[str]] = collections.defaultdict(list)
        lines = iter(raw.splitlines())
        for line in lines:
            if not line.lstrip().startswith("["):
                break
            for i in range(1, len(line), 4):
                if line[i] != " ":
                    self.stacks[i//4 + 1].insert(0, line[i])
        next(lines)
        self.instrs: list[Instruction] = []
        for line in lines:
            parts = re.match(r"move (\d+) from (\d+) to (\d+)", line)
            assert parts is not None
            self.instrs.append(Instruction(int(parts[1]), int(parts[2]), int(parts[3])))

    def get_top(self, stacks: dict[int, list[str]]) -> str:
        """Get the top crates."""
        return "".join([stacks[i][-1] for i in range(1, len(stacks)+1)])

    def initial_stacks(self) -> dict[int, list[str]]:
        """Get a copy of the initial stacks."""
        return {k: list(v) for k, v in self.stacks.items()}

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        stacks = self.initial_stacks()
        for instr in self.instrs:
            for _ in range(instr.count):
                stacks[instr.dest].append(stacks[instr.source].pop())
        return self.get_top(stacks)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        stacks = self.initial_stacks()
        for instr in self.instrs:
            stacks[instr.dest].extend(stacks[instr.source][-instr.count:])
            del stacks[instr.source][-instr.count:]
        return self.get_top(stacks)


if __name__ == "__main__":
    Day.submit()
