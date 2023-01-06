"""The solution to day 23."""
from .aoc_helper import Solution


# MOVES is { surroundings: (north, south, west, east) }, where:
#  - surroundings is a bitfield of the neighbours, with the following layout:
#        765
#        4 3  -->  76543210   (1 means neighbour, 0 means no neighbour)
#        210
#  - north, south, west, east are (dx, dy) pairs for that direction, or None if
#    that direction is blocked
# Since surroundings ranges from 0 to 255, we store MOVES as a 256-element list,
# where the index is the surroundings.
MOVES = [
    (
        (0, -1) if not (
            (surroundings & (1 << 7))
            | (surroundings & (1 << 6))
            | (surroundings & (1 << 5))
        ) else None,
        (0, 1) if not (
            (surroundings & (1 << 2))
            | (surroundings & (1 << 1))
            | (surroundings & (1 << 0))
        ) else None,
        (-1, 0) if not (
            (surroundings & (1 << 7))
            | (surroundings & (1 << 4))
            | (surroundings & (1 << 2))
        ) else None,
        (1, 0) if not (
            (surroundings & (1 << 5))
            | (surroundings & (1 << 3))
            | (surroundings & (1 << 0))
        ) else None,
    ) for surroundings in range(1 << 8)
]
MOVES[0] = (None, None, None, None)  # Do nothing if there are no neighbours.

# SURROUNDINGS is a list of (dx, dy) pairs for the 8 surrounding cells, such
# that the index of the pair is the bit index in the surroundings bitfield.
SURROUNDINGS = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
]

Propositions = dict[tuple[int, int], tuple[int, int]]
Cells = set[tuple[int, int]]


class Day(Solution):
    """The solution to day 23."""

    day = 23

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        cells = [[cell == "#" for cell in line] for line in raw.splitlines()]
        self.cells = {(x, y) for y, row in enumerate(cells) for x, cell in enumerate(row) if cell}

    def get_propositions(self, cells: Cells, round: int) -> Propositions:
        """Find all movement propositions, and the cells they move from."""
        propositions = Propositions()
        for x, y in cells:
            surroundings = 0
            for i, (dx, dy) in enumerate(SURROUNDINGS):
                surroundings |= ((x + dx, y + dy) in cells) << i
            moves = MOVES[surroundings]
            for i in range(4):
                if move := moves[(i + round) % 4]:
                    target = x + move[0], y + move[1]
                    if target in propositions:
                        # A maximum of two elves might make the same proposition.
                        del propositions[target]
                    else:
                        propositions[target] = x, y
                    break
        return propositions

    def apply_propositions(self, cells: Cells, propositions: Propositions) -> Cells:
        """Apply valid propositions to the cells."""
        for (x, y), move_from in propositions.items():
            cells.add((x, y))
            cells.remove(move_from)
        return cells

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        cells = self.cells.copy()
        for round in range(10):
            propositions = self.get_propositions(cells, round)
            cells = self.apply_propositions(cells, propositions)
        min_x, max_x = min(x for x, _ in cells), max(x for x, _ in cells)
        min_y, max_y = min(y for _, y in cells), max(y for _, y in cells)
        return (max_x - min_x + 1) * (max_y - min_y + 1) - len(cells)  

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        cells = self.cells.copy()
        round = 0
        while True:
            propositions = self.get_propositions(cells, round)
            round += 1
            if not propositions:
                return round
            cells = self.apply_propositions(cells, propositions)


if __name__ == "__main__":
    Day.submit()
