"""The solution to day 9."""
from .aoc_helper import Solution


class Vec2:
    """An x, y vector."""

    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __add__(self, other: "Vec2") -> "Vec2":
        return Vec2(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Vec2") -> "Vec2":
        return Vec2(self.x - other.x, self.y - other.y)

    def __repr__(self) -> str:
        return f"Vec2({self.x}, {self.y})"

    def __str__(self) -> str:
        return f"Vec2({self.x}, {self.y})"

    def freeze(self) -> tuple[int, int]:
        return self.x, self.y

    def move_towards(self, other: "Vec2"):
        """Move towards the another position by max. 1 unit in each axis, unless already touching."""
        offset = self - other
        if -1 <= offset.x <= 1 and -1 <= offset.y <= 1:
            return
        if self.y < other.y:
            self.y += 1
        elif self.y > other.y:
            self.y -= 1
        if self.x < other.x:
            self.x += 1
        elif self.x > other.x:
            self.x -= 1


class Day(Solution):
    """The solution to day 9."""

    day = 9

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.movements: list[Vec2] = []
        for line in raw.split("\n"):
            raw_dir, raw_count = line.split()
            dir = {
                "U": Vec2(0, 1),
                "D": Vec2(0, -1),
                "L": Vec2(-1, 0),
                "R": Vec2(1, 0),
            }[raw_dir]
            for _ in range(int(raw_count)):
                self.movements.append(dir)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        tail = Vec2(0, 0)
        head = Vec2(0, 0)
        visited: set[tuple[int, int]] = {tail.freeze()}
        for delta in self.movements:
            head += delta
            tail.move_towards(head)
            visited.add(tail.freeze())
        return len(visited)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        knots = [Vec2(0, 0) for _ in range(10)]
        visited: set[tuple[int, int]] = {(0, 0)}
        for delta in self.movements:
            knots[0] += delta
            for head, tail in zip(knots, knots[1:]):
                tail.move_towards(head)
            visited.add(knots[-1].freeze())
        return len(visited)


if __name__ == "__main__":
    Day.submit()
