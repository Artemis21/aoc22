"""The solution to day 1."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 1."""

    day = 1

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.data = list(map(lambda sect: sum(map(int, sect.split())), raw.split("\n\n")))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return max(self.data)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        a, b, c = 0, 0, 0
        for i in self.data:
            if i > c:
                if i > b:
                    if i > a:
                        a, b, c = i, a, b
                    else:
                        b, c = i, b
                else:
                    c = i
        return a + b + c


if __name__ == "__main__":
    Day.submit()
