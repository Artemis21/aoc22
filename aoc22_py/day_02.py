"""The solution to day 2."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 2."""

    day = 2

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.fields = [line.split() for line in raw.splitlines()]

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        total = 0
        for opp, me in self.fields:
            score = "XYZ".index(me)
            opscore = "ABC".index(opp)
            if score == (opscore + 1) % 3:
                total += 6
            elif score == opscore:
                total += 3
            total += score + 1
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        total = 0
        for opp, me in self.fields:
            opscore = "ABC".index(opp)
            if me == "X":
                total += 1 + (opscore - 1) % 3
            elif me == "Y":
                total += 4 + opscore
            else:
                total += 7 + (opscore + 1) % 3
        return total


if __name__ == "__main__":
    Day.submit()
