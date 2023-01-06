"""The solution to day 3."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 3."""

    day = 3

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.lines = raw.splitlines()

    def item_priority(self, item: str) -> int:
        """Get the priority value of an item."""
        if item.islower():
            return ord(item) - ord('a') + 1
        return ord(item) - ord('A') + 27

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        total = 0
        for line in self.lines:
            a = line[0:len(line)//2]
            b = line[len(line)//2:]
            inter = (set(a) & set(b)).pop()
            total += self.item_priority(inter)
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        total = 0
        for a, b, c in [self.lines[i:i+3] for i in range(0, len(self.lines), 3)]:
            inter = (set(a) & set(b) & set(c)).pop()
            total += self.item_priority(inter)
        return total


if __name__ == "__main__":
    Day.submit()
