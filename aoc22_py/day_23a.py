"""The solution to day 23."""
from .aoc_helper import Solution


def proposal(surroundings: list[list[bool]], direction_offset: int) -> tuple[int, int] | None:
    """Determine where an elf with the given surroundings will propose to move.
    
    Surroundings is a list of three lists of three bools, the neighbours and the elf
    itself in the following layout:
    
        0,0 0,1 0,2
        1,0 1,1 1,2
        2,0 2,1 2,2

    The elf may be false if there is no elf.
    """
    if not surroundings[1][1]:
        return None  # Do nothing if there is no elf.
    if sum(cell for row in surroundings for cell in row) == 1:
        return None  # Do nothing if there are no neighbours.
    moves = [
        None if any(surroundings[0]) else (0, -1),
        None if any(surroundings[2]) else (0, 1),
        None if any(row[0] for row in surroundings) else (-1, 0),
        None if any(row[2] for row in surroundings) else (1, 0),
    ]
    moves = moves[direction_offset:] + moves[:direction_offset]
    return moves[0] or moves[1] or moves[2] or moves[3]


def unpack_surroundings(surroundings: int, indices: list[list[int | bool]]) -> list[list[bool]]:
    """Unpack a bitfield of surroundings into a list of lists of bools given certain indices and constants."""
    return [
        [
            index if isinstance(index, bool) else bool(surroundings & (1 << index))
            for index in row
        ]
        for row in indices
    ]


def movement(surroundings: int, direction_offset: int) -> tuple[int, int]:
    """Determine where an elf with the given surroundings will move.
    
    Surroundings is a bitfield of the neighbours, with the following layout:
    
              00 01 02
              03 04 05
        06 07 08 09 10 11 12
        13 14 15    16 17 18
        19 20 21 22 23 24 25
              26 27 28
              29 30 31
    """
    OTHER_ELVES = [
        (
            [
                [0, 1, 2],
                [3, 4, 5],
                [8, 9, 10],
            ],
            (0, -2)
        ),
        (
            [
                [21, 22, 23],
                [26, 27, 28],
                [29, 30, 31]
            ],
            (0, 2)
        ),
        (
            [
                [6, 7, 8],
                [13, 14, 15],
                [19, 20, 21]
            ],
            (-2, 0)
        ),
        (
            [
                [10, 11, 12],
                [16, 17, 18],
                [23, 24, 25]
            ],
            (2, 0)
        ),
    ]
    proposals: list[tuple[int, int]] = []
    for surrounding_indices, offset in OTHER_ELVES:
        elf_surroundings = unpack_surroundings(surroundings, surrounding_indices)
        if elf_proposal := proposal(elf_surroundings, direction_offset):
            proposals.append((elf_proposal[0] + offset[0], elf_proposal[1] + offset[1]))
    self_surroundings = unpack_surroundings(surroundings, [
        [8, 9, 10],
        [15, True, 16],
        [21, 22, 23]
    ])
    if self_proposal := proposal(self_surroundings, direction_offset):
        if self_proposal not in proposals:
            return self_proposal
    return 0, 0


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
    ) for surroundings in range(1 << 32)
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
