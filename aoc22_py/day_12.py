"""The solution to day 12."""
from typing import Iterator

from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 12."""

    day = 12

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.cells = [list(line) for line in raw.splitlines()]

    def height_at(self, x: int, y: int) -> int:
        """Return the height of the cell at (x, y)."""
        value = self.cells[y][x]
        if value == "S":
            value = "a"
        if value == "E":
            value = "z"
        return ord(value)

    def is_end(self, x: int, y: int) -> bool:
        """Return whether the cell at (x, y) is the end."""
        return self.cells[y][x] == "E"

    def nearby(self, x: int, y: int) -> Iterator[tuple[int, int, int]]:
        """Iterate over adjacent cells (x, y, height)."""
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            y1 = y + dy
            x1 = x + dx
            if 0 <= y1 < len(self.cells) and 0 <= x1 < len(self.cells[y1]):
                yield x1, y1, self.height_at(x1, y1)

    def find_all(self, matching: str) -> set[tuple[int, int]]:
        """Return the coords of all cells whose value is in `matching`."""
        return {
            (x, y)
            for y in range(len(self.cells))
            for x in range(len(self.cells[y]))
            if self.cells[y][x] in matching
        }

    def shortest_path(self, start_at_any: str) -> int:
        """Get the shortest path from any cell in `start_at_any` to the end."""
        open = self.find_all(start_at_any)
        visited: set[tuple[int, int]] = set()
        steps = 0
        while True:
            new_open: set[tuple[int, int]] = set()
            for x, y in open:
                visited.add((x, y))
                if self.is_end(x, y):
                    return steps
                current = self.height_at(x, y)
                for x1, y1, other in self.nearby(x, y):
                    if other <= current + 1 and (x1, y1) not in visited:
                        new_open.add((x1, y1))
            open = new_open
            steps += 1

    def shortest_path_backwards(self, start_at_any: str) -> int:
        """Get the shortest path from any cell in `start_at_any` to the end."""
        open = self.find_all("E")
        visited: set[tuple[int, int]] = set()
        steps = 0
        while True:
            new_open: set[tuple[int, int]] = set()
            for x, y in open:
                visited.add((x, y))
                if self.cells[y][x] in start_at_any:
                    return steps
                current = self.height_at(x, y)
                for x1, y1, other in self.nearby(x, y):
                    if current <= other + 1 and (x1, y1) not in visited:
                        new_open.add((x1, y1))
            open = new_open
            steps += 1

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return self.shortest_path_backwards("S")

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        return self.shortest_path_backwards("Sa")


if __name__ == "__main__":
    Day.submit()
