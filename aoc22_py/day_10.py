"""The solution to day 10."""
from advent_of_code_ocr import convert_array_6

from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 10."""

    day = 10

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        x = 1
        self.values: list[int] = []
        for line in raw.split("\n"):
            if not line.strip():
                continue
            self.values.append(x)
            if line != "noop":
                self.values.append(x)
                x += int(line.split()[1])

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return sum(
            x_reg * (clock + 1)
            for clock, x_reg in enumerate(self.values)
            if clock % 40 == 19
        )

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        screen = [[False] * 40 for _ in range(6)]
        for clock, x_reg in enumerate(self.values):
            row, col = divmod(clock, 40)
            if -1 <= col - x_reg <= 1:
                screen[row][col] = True
        return convert_array_6(screen, fill_pixel=True, empty_pixel=False)


if __name__ == "__main__":
    Day.submit()
