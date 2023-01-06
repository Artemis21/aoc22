"""The solution to day 4."""
import re

from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 4."""

    day = 4

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.fields = [list(map(int, re.findall(r"\d+", line))) for line in raw.splitlines()]

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return sum(
            (a <= x and y <= b) or (x <= a and b <= y)
            for a, b, x, y in self.fields
        )

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        return sum(x <= b and y >= a for a, b, x, y in self.fields)


if __name__ == "__main__":
    Day.submit()
