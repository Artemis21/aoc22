"""The solution to day 15."""
import re

from .aoc_helper import Solution


class Day(Solution):
    """The solution to day 15."""

    day = 15

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.sensors: list[tuple[int, int, int, int]] = []
        for line in raw.split("\n"):
            match = re.match(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", line)
            if not match:
                raise ValueError(f"Invalid line: {line}")
            xs, ys, xb, yb = map(int, match.groups())
            self.sensors.append((xs, ys, xb, yb))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        ya = 2_000_000
        covered_ranges: list[tuple[int, int]] = []
        for xs, ys, xb, yb in self.sensors:
            max_x_dist = abs(xs - xb) + abs(ys - yb) - abs(ys - ya)
            if max_x_dist > 0:
                covered_ranges.append((xs - max_x_dist, xs + max_x_dist))
        points_of_interest: list[tuple[int, bool]] = []
        for start_x, end_x in covered_ranges:
            points_of_interest.append((start_x, True))
            points_of_interest.append((end_x, False))
        points_of_interest.sort()
        total_points = 0
        open_ranges = 0
        opened_at = None
        for point, is_start in points_of_interest:
            if is_start:
                if open_ranges == 0:
                    assert opened_at is None
                    opened_at = point
                open_ranges += 1
            else:
                open_ranges -= 1
                if open_ranges == 0:
                    assert opened_at is not None
                    total_points += point - opened_at
                    opened_at = None
        return total_points

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        size = 4_000_000
        yaa = None
        for ya in range(size + 1):
            covered_ranges: list[tuple[int, int]] = []
            for xs, ys, xb, yb in self.sensors:
                max_x_dist = abs(xs - xb) + abs(ys - yb) - abs(ys - ya)
                if max_x_dist > 0:
                    covered_ranges.append((xs - max_x_dist, xs + max_x_dist))
            points_of_interest: list[tuple[int, bool]] = []
            for start_x, end_x in covered_ranges:
                points_of_interest.append((max(0, min(start_x, size)), True))
                points_of_interest.append((max(0, min(end_x, size)), False))
            points_of_interest.sort()
            total_points = 0
            open_ranges = 0
            opened_at = None
            for point, is_start in points_of_interest:
                if is_start:
                    if open_ranges == 0:
                        assert opened_at is None
                        opened_at = point
                    open_ranges += 1
                else:
                    open_ranges -= 1
                    if open_ranges == 0:
                        assert opened_at is not None
                        total_points += point - opened_at
                        opened_at = None
            if total_points < size:
                yaa = ya
                break
        else:
            raise ValueError("No solution found")
        for xa in range(size + 1):
            for xs, ys, xb, yb in self.sensors:
                if abs(xs - xb) + abs(ys - yb) >= abs(ys - yaa) + abs(xs - xa):
                    break
            else:
                return size * xa + yaa
        raise ValueError("No solution found")

if __name__ == "__main__":
    Day.submit()
