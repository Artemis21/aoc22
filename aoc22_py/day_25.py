"""The solution to day 25."""
from .aoc_helper import Solution


def parse_snafu(snafu: str) -> int:
    """Parse a SNAFU string into a number."""
    return sum(("=-012".index(digit) - 2) * 5 ** place for place, digit in enumerate(snafu[::-1]))


def to_snafu(number: int) -> str:
    """Convert a number to a SNAFU string."""
    snafu: list[str] = []
    while number:
        new_number = round(number / 5)
        digit = number - new_number * 5
        snafu.append("=-012"[digit + 2])
        number = new_number
    return "".join(snafu[::-1])


class Day(Solution):
    """The solution to day 25."""

    day = 25

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.lines = raw.splitlines()

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return to_snafu(sum(map(parse_snafu, self.lines)))

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""


if __name__ == "__main__":
    Day.submit()
