"""The solution to day 18."""
from collections import defaultdict
from itertools import product

from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 18."""

    day = 18

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        # [ (x, y, z) ]
        self.cubes: list[tuple[int, int, int]] = [tuple(map(int, line.split(","))) for line in raw.splitlines()]

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        return self.sa_of_scan(self.cubes)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        cells: dict[tuple[int, int, int], bool] = defaultdict(bool)
        for x, y, z in self.cubes:
            cells[(x, y, z)] = True
        min_x = min(x for x, _, _ in self.cubes)
        max_x = max(x for x, _, _ in self.cubes)
        min_y = min(y for _, y, _ in self.cubes)
        max_y = max(y for _, y, _ in self.cubes)
        min_z = min(z for _, _, z in self.cubes)
        max_z = max(z for _, _, z in self.cubes)
        visited: set[tuple[int, int, int]] = set()
        open: set[tuple[int, int, int]] = set(product((min_x - 1, max_x + 1), (min_y - 1, max_y + 1), (min_z - 1, max_z + 1)))
        while open:
            x, y, z = open.pop()
            if (x, y, z) in visited:
                continue
            visited.add((x, y, z))
            neighbours = [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            for x, y, z in neighbours:
                if x < min_x - 1 or x > max_x + 1 or y < min_y - 1 or y > max_y + 1 or z < min_z - 1 or z > max_z + 1:
                    continue
                neighbour = x, y, z
                if neighbour in open or neighbour in visited:
                    continue
                if not cells[neighbour]:
                    open.add(neighbour)
        filled_scan: list[tuple[int, int, int]] = []
        for coords in product(range(min_x, max_x + 1), range(min_y, max_y + 1), range(min_z, max_z + 1)):
            if coords not in visited:
                filled_scan.append(coords)
        return self.sa_of_scan(filled_scan)

    def sa_of_scan(self, scan: list[tuple[int, int, int]]) -> int:
        """Get the surface area (interior and exterior) of a scan."""
        # [ (x, y, z, plane) ] where:
        #    x, y and z are the low-end coordinates of the side
        #    plane is 0, 1 or 2 for xy, xz or yz
        sides: set[tuple[int, int, int, int]] = set()
        for x, y, z in scan:
            cube_sides = [
                (x, y, z, 0),
                (x, y, z, 1),
                (x, y, z, 2),
                (x + 1, y, z, 2),
                (x, y + 1, z, 1),
                (x, y, z + 1, 0),
            ]
            for side in cube_sides:
                if side in sides:
                    sides.remove(side)
                else:
                    sides.add(side)
        return len(sides)

    def print_scan(self, scan: list[tuple[int, int, int]]):
        """Print a scan."""
        min_x = min(x for x, _, _ in scan)
        max_x = max(x for x, _, _ in scan)
        min_y = min(y for _, y, _ in scan)
        max_y = max(y for _, y, _ in scan)
        min_z = min(z for _, _, z in scan)
        max_z = max(z for _, _, z in scan)
        print("Scan is from", min_x, min_y, min_z, "to", max_x, max_y, max_z)
        for z in range(min_z, max_z + 1):
            print(f"z={z}")
            for y in range(min_y, max_y + 1):
                for x in range(min_x, max_x + 1):
                    print("#" if (x, y, z) in scan else ".", end="")
                print()
            print()


if __name__ == "__main__":
    Day.submit()
