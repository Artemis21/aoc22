"""The solution to day 22."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Literal, Protocol, TypeVar, Generic

from .aoc_helper import Solution


Position = TypeVar("Position", covariant=False, contravariant=False, bound="BasicPosition")

FACE_SIZE = 4


class Map(Protocol[Position]):
    """A protocol for a map."""

    def __init__(self, lines: list[list[Literal[".", "#", " "]]]):
        """Initialise the map."""
        ...

    def step_forward(self, position: Position) -> Position:
        """Move forward one step from the given position."""
        ...

    def is_wall(self, position: Position) -> bool:
        """Return whether the given position is a wall."""
        ...

    def starting_position(self) -> Position:
        """Find the starting position."""
        ...

    def score_position(self, position: Position) -> int:
        """Calculate the score for a given position."""
        ...

    def print_with(self, position: Position):
        """Print the map with a player at the given position."""
        ...


@dataclass
class BasicPosition:
    """A basic position on the map for part 1."""

    x: int
    y: int
    dx: int
    dy: int

    def rotate_right(self) -> BasicPosition:
        """Rotate right."""
        return BasicPosition(x=self.x, y=self.y, dx=-self.dy, dy=self.dx)

    def rotate_left(self) -> BasicPosition:
        """Rotate left."""
        return BasicPosition(x=self.x, y=self.y, dx=self.dy, dy=-self.dx)

    def next_position(self) -> BasicPosition:
        """Get the next position after moving forward."""
        return BasicPosition(
            x=self.x + self.dx,
            y=self.y + self.dy,
            dx=self.dx,
            dy=self.dy,
        )


@dataclass
class FacePosition(BasicPosition):
    """A position that takes in to account the faces of the cube, for part 2."""

    face: Face

    @classmethod
    def from_basic(cls, basic: BasicPosition, face: Face) -> FacePosition:
        """Create a face position from a face and a basic position on that face."""
        return FacePosition(x=basic.x, y=basic.y, dx=basic.dx, dy=basic.dy, face=face)

    def rotate_right(self) -> FacePosition:
        """Rotate right."""
        return self.from_basic(super().rotate_right(), self.face)

    def rotate_left(self) -> FacePosition:
        """Rotate left."""
        return self.from_basic(super().rotate_left(), self.face)

    def next_position(self) -> FacePosition:
        """Get the next position after moving forward."""
        next = super().next_position()
        if next.x < 0:
            face = self.face.adjacent[0]
            clockwise_distance = FACE_SIZE - 1 - self.y
        elif next.y < 0:
            face = self.face.adjacent[1]
            clockwise_distance = self.x
        elif next.x >= FACE_SIZE:
            face = self.face.adjacent[2]
            clockwise_distance = self.y
        elif next.y >= FACE_SIZE:
            face = self.face.adjacent[3]
            clockwise_distance = FACE_SIZE - 1 - self.x
        else:
            return self.from_basic(next, self.face)
        dx, dy = face.walk_from(self.face)
        match (dx, dy):
            case (-1, 0):
                x = FACE_SIZE - 1
                y = FACE_SIZE - 1 - clockwise_distance
            case (0, -1):
                x = clockwise_distance
                y = FACE_SIZE - 1
            case (1, 0):
                x = 0
                y = clockwise_distance
            case (0, 1):
                x = FACE_SIZE - 1 - clockwise_distance
                y = 0
            case _:
                raise ValueError(f"Unexpected direction: {(dx, dy)}.")
        return FacePosition(x=x, y=y, dx=dx, dy=dy, face=face)


# (-1,  0) -> (-1,  0): (max_x,     y)
# (-1,  0) -> ( 0, -1): (s - y, max_y)
# (-1,  0) -> ( 1,  0): (min_x, s - y)
# (-1,  0) -> ( 0,  1): (    y, min_y)

# ( 0, -1) -> (-1,  0): (max_x, s - x)
# ( 0, -1) -> ( 0, -1): (    x, max_y)
# ( 0, -1) -> ( 1,  0): (min_x,     x)
# ( 0, -1) -> ( 0,  1): (s - x, min_y)

# ( 1,  0) -> (-1,  0): (max_x, s - y)
# ( 1,  0) -> ( 0, -1): (    y, max_y)
# ( 1,  0) -> ( 1,  0): (min_x,     y)
# ( 1,  0) -> ( 0,  1): (s - y, min_y)

# ( 0,  1) -> (-1,  0): (max_x,     x)
# ( 0,  1) -> ( 0, -1): (s - x, max_y)
# ( 0,  1) -> ( 1,  0): (min_x, s - x)
# ( 0,  1) -> ( 0,  1): (    x, min_y)


class BasicMap:
    """A 2d map for part 1."""

    def __init__(self, lines: list[list[Literal[".", "#", " "]]]):
        """Set up the map."""
        self.lines = lines

    def step_forward(self, position: BasicPosition) -> BasicPosition:
        """Step forward one move."""
        next = position.next_position()
        next.y %= len(self.lines)
        next.x %= len(self.lines[next.y])
        while self.lines[next.y][next.x] == " ":
            next = next.next_position()
            next.y %= len(self.lines)
            next.x %= len(self.lines[next.y])
        return next

    def is_wall(self, position: BasicPosition) -> bool:
        """Check if a given position contains a wall."""
        char = self.lines[position.y][position.x]
        assert char in "#."  # A position pointing to a space should never be exposed.
        return char == "#"

    def starting_position(self) -> BasicPosition:
        """Get the starting position."""
        x = self.lines[0].index(".")
        return BasicPosition(x=x, y=0, dx=1, dy=0)

    def score_position(self, position: BasicPosition) -> int:
        """Calculate the score for a given position."""
        match position.dx, position.dy:
            case 1, 0:
                facing = 0
            case 0, 1:
                facing = 1
            case -1, 0:
                facing = 2
            case 0, -1:
                facing = 3
            case _:
                raise ValueError("Invalid direction")
        return (position.y + 1) * 1000 + (position.x + 1) * 4 + facing

    def print_with(self, position: BasicPosition):
        """Print the map with a player at the given position."""
        match position.dx, position.dy:
            case 1, 0:
                facing = ">"
            case 0, 1:
                facing = "v"
            case -1, 0:
                facing = "<"
            case 0, -1:
                facing = "^"
            case _:
                raise ValueError("Invalid direction")
        for y, line in enumerate(self.lines):
            for x, char in enumerate(line):
                if (x, y) == (position.x, position.y):
                    print(end=facing)
                else:
                    print(end=char)
            print()


class CubeMap:
    """A map on the surface of a cube for part 2."""

    def __init__(self, lines: list[list[Literal[".", "#", " "]]]):
        """Set up the map."""
        faces_by_position = {
            (x, y): PartialFace((x, y)) if lines[y * FACE_SIZE][x * FACE_SIZE] != " " else None
            for y in range(len(lines) // FACE_SIZE)
            for x in range(len(lines[0]) // FACE_SIZE)
        }
        for (x, y), face in faces_by_position.items():
            if face is None:
                continue
            for side, (dx, dy) in enumerate(((-1, 0), (0, -1), (1, 0), (0, 1))):
                if (x + dx, y + dy) in faces_by_position:
                    face.adjacent[side] = faces_by_position[x + dx, y + dy]
        cube = PartialCube()
        cube.place(next(face for face in faces_by_position.values() if face), 0)
        self.faces = {face.position: face for face in cube.fill_faces()}
        self.net_map = BasicMap(lines)

    def _position_to_basic(self, position: FacePosition) -> BasicPosition:
        """Convert a position on a face to a basic position on the net map."""
        face_x, face_y = position.face.position
        x = face_x * FACE_SIZE + position.x
        y = face_y * FACE_SIZE + position.y
        return BasicPosition(x=x, y=y, dx=position.dx, dy=position.dy)

    def step_forward(self, position: FacePosition) -> FacePosition:
        """Step forward one move."""
        return position.next_position()

    def is_wall(self, position: FacePosition) -> bool:
        """Check if a given position contains a wall."""
        basic = self._position_to_basic(position)
        return self.net_map.is_wall(basic)

    def starting_position(self) -> FacePosition:
        """Get the starting position."""
        basic = self.net_map.starting_position()
        face_x, x = divmod(basic.x, FACE_SIZE)
        face_y, y = divmod(basic.y, FACE_SIZE)
        face = self.faces[(face_x, face_y)]
        return FacePosition(x=x, y=y, dx=basic.dx, dy=basic.dy, face=face)

    def score_position(self, position: FacePosition) -> int:
        """Calculate the score for a given position."""
        basic = self._position_to_basic(position)
        return self.net_map.score_position(basic)

    def print_with(self, position: FacePosition):
        """Print the map with a player at the given position."""
        basic = self._position_to_basic(position)
        self.net_map.print_with(basic)


@dataclass
class Agent(Generic[Position]):
    """The character moving through the map."""

    instructions: list[int | Literal["L", "R"]]
    map: Map[Position]
    position: Position

    def __init__(self, map: Map[Position], instructions: list[int | Literal["L", "R"]]):
        """Set up the agent at the starting position."""
        self.map = map
        self.instructions = instructions
        self.position = map.starting_position()

    def move_n(self, n: int) -> None:
        """Move forward n steps, or until we hit a wall."""
        for _ in range(n):
            position = self.map.step_forward(self.position)
            if self.map.is_wall(position):
                break
            self.position = position

    def follow_instructions(self) -> None:
        """Follow the instructions."""
        for instruction in self.instructions:
            match instruction:
                case int(steps):
                    self.move_n(steps)
                case "L":
                    self.position = self.position.rotate_left()
                case "R":
                    self.position = self.position.rotate_right()
            self.map.print_with(self.position)


class PartialFace:
    """A face of a cube that does not yet know all its adjacent faces."""

    def __init__(self, position: tuple[int, int]):
        self.position = position
        self.adjacent: list[PartialFace | None] = [None] * 4


class Face:
    """A face of a cube that knows all its adjacent faces."""

    def __init__(self, position: tuple[int, int], adjacent: list[Face]):
        self.position = position
        self.adjacent = adjacent

    def walk_from(self, face: Face) -> tuple[int, int]:
        """Get the cardinal direction you would be walking if you walked on to this face from another face."""
        return [
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
        ][self.adjacent.index(face)]

    def __str__(self) -> str:
        """Display the face."""
        return f"Face{self.position} -> {[face.position for face in self.adjacent]}"

    def __repr__(self) -> str:
        """Display the face."""
        return f"Face(position={self.position}, adjacent={[face.position for face in self.adjacent]})"


ADJACENT_FACES = [
    (2, 4, 3, 5),  #  [F]ront: LTRB
    (2, 5, 3, 4),  #   b[A]ck: LBRT
    (0, 5, 1, 4),  #   [L]eft: FBAT
    (0, 4, 1, 5),  #  [R]ight: FTAB
    (0, 2, 1, 3),  #    [T]op: FLAR
    (0, 3, 1, 2),  # [B]ottom: FRAL
]


class PartialCube:
    def __init__(self):
        # 0: front, 1: back, 2: left, 3: right, 4: top, 5: bottom
        self.faces: list[PartialFace | None] = [None] * 6

    def place(self, face: PartialFace, side: int):
        """Place a new face on to the cube."""
        self.faces[side] = face
        adjacent_faces = ADJACENT_FACES[side]
        for idx, face_idx in enumerate(adjacent_faces):
            other_face = self.faces[face_idx]
            if other_face and other_face in face.adjacent:
                offset = face.adjacent.index(other_face) - idx
                break
        else:
            offset = 0
        for idx, face_idx in enumerate(adjacent_faces):
            new_face = face.adjacent[(idx + offset) % len(face.adjacent)]
            if new_face and not self.faces[face_idx]:
                self.place(new_face, face_idx)

    def fill_faces(self) -> list[Face]:
        """Turn the partial faces in to full faces, once all faces have been placed."""
        assert all(self.faces)
        partial_faces: list[PartialFace] = self.faces  # type: ignore
        faces: list[Face] = [Face(face.position, adjacent=[]) for face in partial_faces]
        for idx, (face, adjacent_faces) in enumerate(zip(partial_faces, ADJACENT_FACES)):
            offset = None
            for off, face_idx in enumerate(adjacent_faces):
                other_face = partial_faces[face_idx]
                if other_face in face.adjacent:
                    offset = off - face.adjacent.index(other_face)
                    break
            assert offset is not None
            for face_idx in adjacent_faces:
                faces[idx].adjacent.append(faces[face_idx])
            faces[idx].adjacent = faces[idx].adjacent[offset:] + faces[idx].adjacent[:offset]
        return faces


class Day(Solution):
    """The solution to day 22."""

    day = 22

    def __init__(self, raw: str):
        """Parse and store the raw data."""
        raw_map, raw_instructions = raw.split("\n\n")
        self.map: list[list[Literal[".", "#", " "]]] = [list(line) for line in raw_map.splitlines()]  # type: ignore
        max_len = max(map(len, self.map))
        for row in self.map:
            row.extend([" "] * (max_len - len(row)))
        self.instructions: list[int | Literal["L", "R"]] = []
        instr_buffer = ""
        for char in raw_instructions:
            if char.isdigit():
                instr_buffer += char
            elif instr_buffer:
                self.instructions.append(int(instr_buffer))
                instr_buffer = ""
            if char == "L" or char == "R":
                self.instructions.append(char)
        if instr_buffer:
            self.instructions.append(int(instr_buffer))

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        map = BasicMap(self.map)
        agent = Agent(map, self.instructions)
        agent.follow_instructions()
        return map.score_position(agent.position)

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        map = CubeMap(self.map)
        agent = Agent(map, self.instructions)
        agent.follow_instructions()
        return map.score_position(agent.position)


if __name__ == "__main__":
    Day.submit()
