"""The solution to day 24."""
from __future__ import annotations

import heapq
from functools import cache
from dataclasses import dataclass

from .aoc_helper import Solution


@dataclass(frozen=True)
class Blizzard:
    """A blizzard."""

    x: int
    y: int
    dx: int
    dy: int
    width: int
    height: int

    @classmethod
    def from_ascii(cls, ascii: str, *, x: int, y: int, width: int, height: int) -> Blizzard:
        """Parse a blizzard from an ASCII art character."""
        match ascii:
            case "^":
                dx, dy = 0, -1
            case "<":
                dx, dy = -1, 0
            case ">":
                dx, dy = 1, 0
            case "v":
                dx, dy = 0, 1
            case _:
                raise ValueError(f"Unknown blizzard character: {ascii}")
        return Blizzard(x=x, y=y, dx=dx, dy=dy, width=width, height=height)

    def sequel(self) -> Blizzard:
        """Determine what the blizzard will become next."""
        return Blizzard(
            x=(self.x + self.dx) % self.width,
            y=(self.y + self.dy) % self.height,
            dx=self.dx,
            dy=self.dy,
            width=self.width,
            height=self.height,
        )


class Map:
    """The state of the map at a given time."""

    @classmethod
    def from_ascii(cls, ascii: str) -> Map:
        """Parse a map from ASCII art."""
        lines = ascii.splitlines()
        # Discard top/bottom/left/right padding.
        lines = [line[1:-1] for line in lines[1:-1]]
        width, height = len(lines[0]), len(lines)
        blizzards = [
            Blizzard.from_ascii(ascii=char, x=x, y=y, width=width, height=height)
            for y, line in enumerate(lines)
            for x, char in enumerate(line)
            if char != "."
        ]
        return cls(blizzards=blizzards)

    def __init__(self, blizzards: list[Blizzard]):
        """Set up the map."""
        self.blizzards = blizzards
        self.occupied = {(b.x, b.y) for b in blizzards}
        self.width = blizzards[0].width
        self.height = blizzards[0].height

    def is_occupied(self, x: int, y: int) -> bool:
        """Determine if the given location is occupied."""
        if x < 0 or x >= self.width:
            return True
        if (y == -1 and x == 0) or (y == self.height and x == self.width - 1):
            return False
        if y < 0 or y >= self.height:
            return True
        return (x, y) in self.occupied

    @cache
    def sequel(self) -> Map:
        """Determine what the map will become next."""
        return Map([b.sequel() for b in self.blizzards])


@dataclass(frozen=True)
class State:
    """The state of the agent at some point in the search."""

    map: Map
    x: int
    y: int
    steps: int
    target_x: int
    target_y: int

    def end(self) -> bool:
        """Determine if the state is final."""
        return self.x == self.target_x and self.y == self.target_y

    def lower_bound(self) -> int:
        """Determine the minimal distance to the end."""
        return abs(self.target_x - self.x) + abs(self.target_y - self.y) + self.steps

    def __lt__(self, other: State) -> bool:
        """Determine if this state is better than another."""
        return (self.lower_bound(), self.steps) < (other.lower_bound(), other.steps)

    def sequels(self) -> list[State]:
        """Determine what possible states come next."""
        map = self.map.sequel()
        return [
            State(
                map=map,
                x=self.x + dx,
                y=self.y + dy,
                steps=self.steps + 1,
                target_x=self.target_x,
                target_y=self.target_y,
            )
            for dx, dy in ((0, -1), (-1, 0), (1, 0), (0, 1), (0, 0))
            if not map.is_occupied(self.x + dx, self.y + dy)
        ]

    def best_path(self) -> State:
        """Determine number of final state of the best path which includes this state."""
        queue = [self]
        visited: set[State] = set()
        while queue:
            state = heapq.heappop(queue)
            if state in visited:
                continue
            visited.add(state)
            if state.end():
                return state
            for sequel in state.sequels():
                heapq.heappush(queue, sequel)
        raise ValueError("No solution found.")


class Day(Solution):
    """The solution to day 24."""

    day = 24

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.map = Map.from_ascii(raw)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        Map.sequel.cache_clear()
        return State(self.map, x=0, y=-1, steps=0, target_x=self.map.width - 1, target_y=self.map.height).best_path().steps

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        Map.sequel.cache_clear()
        a = State(self.map, x=0, y=-1, steps=0, target_x=self.map.width - 1, target_y=self.map.height).best_path()
        b = State(a.map, x=a.x, y=a.y, steps=a.steps, target_x=0, target_y=-1).best_path()
        c = State(b.map, x=b.x, y=b.y, steps=b.steps, target_x=self.map.width - 1, target_y=self.map.height).best_path()
        return c.steps


if __name__ == "__main__":
    Day.submit()
