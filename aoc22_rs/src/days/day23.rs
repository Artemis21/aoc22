use rustc_hash::{FxHashSet, FxHashMap};

use crate::{Day, vec2::Vec2};

#[derive(Clone)]
pub struct Day23(Cells);

#[derive(Clone, Copy)]
struct Neighbours(u8);

#[derive(Clone, Copy)]
enum Neighbour {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

const NEIGHBOURS: [Neighbour; 8] = [
    Neighbour::TopLeft,
    Neighbour::Top,
    Neighbour::TopRight,
    Neighbour::Left,
    Neighbour::Right,
    Neighbour::BottomLeft,
    Neighbour::Bottom,
    Neighbour::BottomRight,
];

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    const fn neighbours(self) -> [Neighbour; 3] {
        match self {
            Direction::Up => [
                Neighbour::TopLeft,
                Neighbour::Top,
                Neighbour::TopRight,
            ],
            Direction::Down => [
                Neighbour::BottomLeft,
                Neighbour::Bottom,
                Neighbour::BottomRight,
            ],
            Direction::Left => [
                Neighbour::TopLeft,
                Neighbour::Left,
                Neighbour::BottomLeft,
            ],
            Direction::Right => [
                Neighbour::TopRight,
                Neighbour::Right,
                Neighbour::BottomRight,
            ],
        }
    }

    const fn index(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    const fn delta(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
        }
    }
}

impl Neighbour {
    const fn index(self) -> usize {
        match self {
            Neighbour::TopLeft => 0,
            Neighbour::Top => 1,
            Neighbour::TopRight => 2,
            Neighbour::Left => 3,
            Neighbour::Right => 4,
            Neighbour::BottomLeft => 5,
            Neighbour::Bottom => 6,
            Neighbour::BottomRight => 7,
        }
    }

    const fn delta(self) -> Vec2 {
        match self {
            Neighbour::TopLeft => Vec2::new(-1, -1),
            Neighbour::Top => Vec2::new(0, -1),
            Neighbour::TopRight => Vec2::new(1, -1),
            Neighbour::Left => Vec2::new(-1, 0),
            Neighbour::Right => Vec2::new(1, 0),
            Neighbour::BottomLeft => Vec2::new(-1, 1),
            Neighbour::Bottom => Vec2::new(0, 1),
            Neighbour::BottomRight => Vec2::new(1, 1),
        }
    }
}

#[derive(Clone, Copy)]
struct Moves(u8);

impl Moves {
    const fn get(self, direction: Direction) -> bool {
        self.0 & (1 << direction.index()) != 0
    }
}

impl Neighbours {
    const fn get(self, neighbour: Neighbour) -> bool {
        self.0 & (1 << neighbour.index()) != 0
    }

    const fn direction_blocked(self, direction: Direction) -> bool {
        let [a, b, c] = direction.neighbours();
        self.get(a) || self.get(b) || self.get(c)
    }

    const fn moves(self) -> Moves {
        let mut moves = 0;
        if !self.direction_blocked(Direction::Up) {
            moves |= 1 << Direction::Up.index();
        }
        if !self.direction_blocked(Direction::Down) {
            moves |= 1 << Direction::Down.index();
        }
        if !self.direction_blocked(Direction::Left) {
            moves |= 1 << Direction::Left.index();
        }
        if !self.direction_blocked(Direction::Right) {
            moves |= 1 << Direction::Right.index();
        }
        if moves == 0b1111 {
            moves = 0;  // Don't move at all if there are no neighbours
        }
        Moves(moves)
    }

    const fn table() -> [Moves; 256] {
        let mut table = [Moves(0); 256];
        let mut maybe_i = Some(0);
        while let Some(i) = maybe_i {
            table[i as usize] = Neighbours(i).moves();
            maybe_i = i.checked_add(1);  // Once we reach 255, this will overflow.
        }
        table
    }
}

const MOVES: [Moves; 256] = Neighbours::table();

#[derive(Clone)]
struct Cells(FxHashSet<Vec2>);

impl Cells {
    fn parse(input: &str) -> Self {
        let cells = input.lines().enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(x, y, _)| Vec2::new(x as isize, y as isize))
            .collect();
        Cells(cells)
    }

    fn get_propositions(&self, round: usize) -> FxHashMap<Vec2, Vec2> {
        let round = round % 4;
        let mut propositions = FxHashMap::default();
        for &elf in &self.0 {
            let mut neighbours = 0;
            for neighbour in &NEIGHBOURS {
                if self.0.contains(&(elf + neighbour.delta())) {
                    neighbours |= 1 << neighbour.index();
                }
            }
            let moves = MOVES[neighbours];
            let mut directions = DIRECTIONS;
            directions.rotate_left(round);
            for direction in &directions {
                if moves.get(*direction) {
                    let proposition = elf + direction.delta();
                    if propositions.contains_key(&proposition) {
                        // Multiple elves cannot move to one cell (and a maximum of two might try).
                        propositions.remove(&proposition);
                    } else {
                        propositions.insert(proposition, elf);
                    }
                    break;
                }
            }
        }
        propositions
    }

    fn apply_propositions(&mut self, propositions: &FxHashMap<Vec2, Vec2>) {
        for (proposition, elf) in propositions {
            self.0.remove(elf);
            self.0.insert(*proposition);
        }
    }

    fn bounding_box(&self) -> (Vec2, Vec2) {
        let mut min = Vec2::new(isize::MAX, isize::MAX);
        let mut max = Vec2::new(isize::MIN, isize::MIN);
        for &cell in &self.0 {
            min.0 = min.0.min(cell.0);
            min.1 = min.1.min(cell.1);
            max.0 = max.0.max(cell.0);
            max.1 = max.1.max(cell.1);
        }
        (min, max)
    }
}

impl Day for Day23 {
    fn parse(input: &str) -> Self {
        Self(Cells::parse(input))
    }

    fn part1(&self) -> String {
        let mut cells = self.0.clone();
        for round in 0..10 {
            cells.apply_propositions(&cells.get_propositions(round));
        }
        let (min, max) = cells.bounding_box();
        let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
        (area - cells.0.len() as isize).to_string()
    }

    fn part2(&self) -> String {
        let mut cells = self.0.clone();
        for round in 0.. {
            let propositions = cells.get_propositions(round);
            if propositions.is_empty() {
                return (round + 1).to_string();
            }
            cells.apply_propositions(&propositions);
        }
        unreachable!()
    }
}
