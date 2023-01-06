"""The solution to day 8."""
from .aoc_helper import Solution
import typing

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


class Day(Solution):
    """The solution to day 8."""

    day = 8

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.trees = [[int(cell) for cell in line] for line in raw.split("\n")]

    def iter_trees(self) -> typing.Iterator[typing.Tuple[int, int, int]]:
        """Iterate over the trees."""
        for y, row in enumerate(self.trees):
            for x, tree in enumerate(row):
                yield x, y, tree

    def follow_line(self, x0: int, y0: int, dx: int, dy: int) -> typing.Iterator[int]:
        """Follow a line."""
        x, y = x0, y0
        while True:
            x += dx
            y += dy
            if x < 0 or x >= len(self.trees[0]) or y < 0 or y >= len(self.trees):
                break
            yield self.trees[y][x]

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return sum(
            any(
                not any(size_n >= tree for size_n in self.follow_line(x, y, dx, dy))
                for dx, dy in DIRECTIONS
            )
            for x, y, tree in self.iter_trees()
        )

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        scores: list[int] = []
        max_x = len(self.trees[0])
        max_y = len(self.trees)
        for x, y, tree in self.iter_trees():
            score = 1
            for dx, dy in DIRECTIONS:
                score *= next(
                    (
                        i + 1 for i, size_n in enumerate(self.follow_line(x, y, dx, dy))
                        if size_n >= tree
                    ),
                    (-(y + (1 if dy==1 else 0)) * dy) % max_y + (-(x + (1 if dx==1 else 0)) * dx) % max_x,
                )
            scores.append(score)
        return max(scores)


if __name__ == "__main__":
    Day.submit()
