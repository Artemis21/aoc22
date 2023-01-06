"""The solution to day 14."""
from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 14."""

    day = 14

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.paths: list[list[tuple[int, int]]] = []
        for line in raw.split("\n"):
            path: list[tuple[int, int]] = []
            for point in line.split(" -> "):
                x, y = point.split(",")
                path.append((int(x), int(y)))
            self.paths.append(path)

    def trace_path_segment(self, src: tuple[int, int], dest: tuple[int, int]) -> list[tuple[int, int]]:
        """Trace a path segment from src to dest."""
        path: list[tuple[int, int]] = []
        x0, y0 = src
        x1, y1 = dest
        if x0 == x1:
            for y in range(min(y0, y1), max(y0, y1) + 1):
                path.append((x0, y))
        elif y0 == y1:
            for x in range(min(x0, x1), max(x0, x1) + 1):
                path.append((x, y0))
        else:
            raise ValueError(f"Invalid path segment: {src} -> {dest}")
        return path

    def trace_path(self, path: list[tuple[int, int]]) -> list[tuple[int, int]]:
        """Trace a path."""
        traced_path: list[tuple[int, int]] = []
        for i in range(len(path) - 1):
            traced_path.extend(self.trace_path_segment(path[i], path[i + 1]))
        return traced_path

    def draw_paths(self) -> tuple[list[list[bool]], int]:
        """Draw the paths on a grid.

        Also returns the x offset used.
        """
        min_x = min(min(x for x, _ in path) for path in self.paths)
        max_x = max(max(x for x, _ in path) for path in self.paths)
        max_y = max(max(y for _, y in path) for path in self.paths) + 1
        min_x -= max_y
        max_x += max_y
        x_offset = -min_x
        max_x = max_x + x_offset
        grid: list[list[bool]] = [[False] * (max_x + 1) for _ in range(max_y + 1)]
        for path in self.paths:
            for x, y in self.trace_path(path):
                grid[y][x + x_offset] = True
        return grid, x_offset

    def get_coord(self, grid: list[list[bool]], x: int, y: int) -> bool:
        """Get a coordinate from the grid, returning False if it is off the grid."""
        if 0 <= y < len(grid) and 0 <= x < len(grid[0]):
            return grid[y][x]
        return False

    def fall_sand(self, grid: list[list[bool]], x: int, y: int, hard_bottom: bool = False) -> bool:
        """Let sand fall from the given position.

        Return true if it settled.
        """
        if self.get_coord(grid, x, y):
            return False
        while 0 <= y < len(grid) and 0 <= x < len(grid[0]):
            if not self.get_coord(grid, x, y + 1):
                y += 1
            elif not self.get_coord(grid, x - 1, y + 1):
                x -= 1
                y += 1
            elif not self.get_coord(grid, x + 1, y + 1):
                x += 1
                y += 1
            else:
                grid[y][x] = True
                return True
        if hard_bottom and y == len(grid):
            grid[y - 1][x] = True
            return True
        return False

    def print_grid_with_sand(self, grid: list[list[bool]], x: int, y: int):
        """Print the grid with sand at the given position."""
        cells = [["#" if c else "." for c in row] for row in grid]
        cells[y][x] = "o"
        print()
        print("\n".join("".join(row) for row in cells))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        grid, x_off = self.draw_paths()
        total = 0
        while self.fall_sand(grid, 500 + x_off, 0):
            total += 1
        return total

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        grid, x_off = self.draw_paths()
        total = 0
        while self.fall_sand(grid, 500 + x_off, 0, True):
            total += 1
        return total


if __name__ == "__main__":
    Day.submit()
