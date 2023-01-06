"""The solution to day 17."""
from itertools import cycle

from .aoc_helper import Solution


class Block:
    """One of the stone shapes."""

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        # Offsets of squares from the bottom left corner, y is 0 at the bottom.
        self.offsets = [
            (x, y)
            for y, line in enumerate(reversed(raw.splitlines()))
            for x, char in enumerate(line)
            if char == "#"
        ]
        self.height = 1 + max(y for _, y in self.offsets)
        self.width = 1 + max(x for x, _ in self.offsets)
        min_y_by_x: dict[int, int] = {}
        min_x_by_y: dict[int, int] = {}
        max_x_by_y: dict[int, int] = {}
        for x, y in self.offsets:
            min_y_by_x[x] = min(min_y_by_x.get(x, 99), y)
            min_x_by_y[y] = min(min_x_by_y.get(y, 99), x)
            max_x_by_y[y] = max(max_x_by_y.get(y, -1), x)
        self.lower_edge = [(x, y) for x, y in min_y_by_x.items()]
        self.left_edge = [(x, y) for y, x in min_x_by_y.items()]
        self.right_edge = [(x, y) for y, x in max_x_by_y.items()]


class Trench:
    """The trench the blocks fall into."""

    def __init__(self, width: int, jets: list[int]):
        """Set up the trench with the given width."""
        self.width = width
        self.jets = list(jets)
        self.cells: set[tuple[int, int]] = set()
        self.height: int = 0
        self.buried_height = 0
        self.jet_index = 0

    def next_jet(self) -> int:
        """Get the next jet."""
        jet = self.jets[self.jet_index]
        self.jet_index = (self.jet_index + 1) % len(self.jets)
        return jet

    @property
    def full_height(self) -> int:
        """The height of the trench, including buried cells."""
        return self.height + self.buried_height

    def advance(self, cells: set[tuple[int, int]], height: int, buried_height_increase: int):
        """Advance the simulation to the state with the given cells and height increase."""
        self.cells = cells
        self.height = height
        self.buried_height += buried_height_increase

    def fall(self, block: Block):
        """Fall the block into the trench."""
        x, y = 2, self.height
        max_x = self.width - block.width
        for _ in range(4):
            x += self.next_jet()
            x = 0 if x < 0 else max_x if x > max_x else x
        while y > 0 and self.is_clear(x, y - 1, block.lower_edge):
            y -= 1
            jet = self.next_jet()
            if self.is_clear(x + jet, y, block.left_edge if jet < 0 else block.right_edge):
                x += jet
            x = 0 if x < 0 else max_x if x > max_x else x
        for x_off, y_off in block.offsets:
            self.cells.add((x + x_off, y + y_off))
        block_height = block.height + y
        self.height = self.height if self.height > block_height else block_height

    def is_clear(self, x: int, y: int, offsets: list[tuple[int, int]]) -> bool:
        """Check if the given offsets are empty at the given coordinates."""
        for x_off, y_off in offsets:
            if (x + x_off, y + y_off) in self.cells:
                return False
        return True

    def clear_buried(self):
        """Get rid of buried cells to reduce memory usage and keep numbers small."""
        """
        cells: set[tuple[int, int]] = {(x, y) for y in range(self.height - 32) for x in range(self.width)}
        for block in BLOCKS:
            visited: set[tuple[int, int]] = set()
            open: list[tuple[int, int]] = [(x, self.height) for x in range(self.width - block.width + 1)]
            while open:
                x, y = open.pop()
                if x < 0 or x > self.width - block.width or y < 0 or y > self.height:
                    continue
                if (x, y) in visited:
                    continue
                visited.add((x, y))
                valid = True
                for x_off, y_off in block.offsets:
                    if (x + x_off, y + y_off) in self.cells:
                        valid = False
                if valid:
                    for x_off, y_off in block.offsets:
                        if (x + x_off, y + y_off) in cells:
                            cells.remove((x + x_off, y + y_off))
                    open.append((x - 1, y))
                    open.append((x + 1, y))
                    open.append((x, y - 1))
                    open.append((x, y + 1))
        if not cells:
            return
        height_buried = min(y for _, y in cells)
        self.cells = {(x, y - height_buried) for x, y in cells}
        self.buried_height += height_buried
        self.height -= height_buried
        """
        visited: set[tuple[int, int]] = set()
        open: list[tuple[int, int]] = [(x, self.height) for x in range(self.width)]
        cells: set[tuple[int, int]] = set()
        while open:
            x, y = open.pop()
            if x < 0 or x >= self.width or y < 0 or y > self.height:
                continue
            if (x, y) in visited:
                continue
            visited.add((x, y))
            if (x, y) in self.cells:
                cells.add((x, y))
            else:
                open.append((x - 1, y))
                open.append((x + 1, y))
                open.append((x, y - 1))
                open.append((x, y + 1))
        if not cells:
            return
        height_buried = min(y for _, y in cells)
        self.cells = {(x, y - height_buried) for x, y in cells}
        self.buried_height += height_buried
        self.height -= height_buried

    def __str__(self) -> str:
        """Display the trench."""
        lines: list[str] = []
        for y in range(self.height):
            line = ""
            for x in range(self.width):
                if (x, y) in self.cells:
                    line += "#"
                else:
                    line += "."
            lines.append(line)
        return "\n".join(reversed(lines))

    def copy(self) -> "Trench":
        """Return a copy of the trench."""
        trench = Trench(self.width, self.jets)
        trench.height = self.height
        trench.buried_height = self.buried_height
        trench.jet_index = self.jet_index
        trench.cells = self.cells.copy()
        return trench

    def print_with_block(self, x: int, y: int, block: Block):
        """Print the trench with the block at the given coordinates."""
        trench = self.copy()
        for x_off, y_off in block.offsets:
            trench.cells.add((x + x_off, y + y_off))
        print(trench)


RAW_BLOCKS = """
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
"""


BLOCKS = [Block(raw) for raw in RAW_BLOCKS.strip("\n").split("\n\n")]


class Day(Solution):
    """The solution to day 17."""

    day = 17

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        self.deltas: list[int] = []
        for c in raw:
            if c == ">":
                self.deltas.append(1)
            elif c == "<":
                self.deltas.append(-1)

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        trench = Trench(7, list(self.deltas))
        for block, _ in zip(cycle(BLOCKS), range(2022)):
            trench.fall(block)
        return trench.height

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        trench = Trench(7, list(self.deltas))
        Cells = tuple[tuple[int, int], ...]
        visited: dict[tuple[int, Cells], tuple[int, int]] = {}
        remaining = 1_000_000_000_000
        while remaining > 0:
            for block in BLOCKS:
                trench.fall(block)
            remaining -= len(BLOCKS)
            if trench.jet_index == 0:
                trench.clear_buried()
                key = (trench.jet_index, tuple(sorted(trench.cells)))
                if key in visited:
                    remaining_then, height_then = visited[key]
                    period = remaining_then - remaining
                    increment = trench.full_height - height_then
                    periods, remaining = divmod(remaining, period)
                    trench.buried_height += periods * increment
                    break
                visited[key] = remaining, trench.full_height
        for block, _ in zip(cycle(BLOCKS), range(remaining)):
            trench.fall(block)
        return trench.full_height


if __name__ == "__main__":
    Day.submit()
