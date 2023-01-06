"""The solution to day 6."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 6."""

    day = 6

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.raw = raw

    def first_distinct(self, n: int):
        """Get the index after the first n-length substring of distinct letters."""
        for i in range(len(self.raw)):
            window = self.raw[i:i+n]
            for j, char in enumerate(window):
                if char in window[j+1:]:
                    break
            else:
                return i + n

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return self.first_distinct(4)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        return self.first_distinct(14)


if __name__ == "__main__":
    Day.submit()
