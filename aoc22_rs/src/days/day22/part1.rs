use crate::vec2::Vec2;

use super::{face_size, Map, Tile, Turn};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BasicPosition {
    pub pos: Vec2,
    pub dir: Vec2,
}

#[derive(Clone, Debug)]
pub struct BasicMap {
    pub map: Vec<Vec<Tile>>,
    width: isize,
    height: isize,
    pub face_size: usize,
}

impl BasicMap {
    pub fn parse(raw: &str) -> Self {
        let width = raw.lines().map(str::len).max().unwrap();
        let lines: Vec<Vec<_>> = raw
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::from_char)
                    .chain(std::iter::repeat(Tile::Void))
                    .take(width)
                    .collect()
            })
            .collect();
        let height = lines.len();
        Self {
            map: lines,
            width: width as isize,
            height: height as isize,
            face_size: face_size(width, height),
        }
    }
}

impl Map for BasicMap {
    type Position = BasicPosition;

    fn next_position(&self, mut pos: Self::Position) -> Self::Position {
        let step = |mut pos: Self::Position, factor: usize| {
            pos.pos += pos.dir * factor;
            if pos.pos.0 < 0 {
                pos.pos.0 += self.width;
            } else if pos.pos.0 >= self.width {
                pos.pos.0 -= self.width;
            } else if pos.pos.1 < 0 {
                pos.pos.1 += self.height;
            } else if pos.pos.1 >= self.height {
                pos.pos.1 -= self.height;
            }
            pos
        };
        pos = step(pos, 1);
        while self.tile_at(pos) == Tile::Void {
            pos = step(pos, self.face_size);
        }
        pos
    }

    fn turn_position(pos: Self::Position, turn: Turn) -> Self::Position {
        let dir = match turn {
            Turn::Left => Vec2::new(pos.dir.1, -pos.dir.0),
            Turn::Right => Vec2::new(-pos.dir.1, pos.dir.0),
        };
        Self::Position { dir, ..pos }
    }

    fn tile_at(&self, pos: Self::Position) -> Tile {
        *pos.pos.index(&self.map)
    }

    fn start_point(&self) -> Self::Position {
        (0..self.width as usize)
            .find(|&x| self.map[0][x] == Tile::Open)
            .map(|x| BasicPosition {
                pos: Vec2::new(x as isize, 0),
                dir: Vec2::new(1, 0),
            })
            .unwrap()
    }

    fn score_position(&self, pos: Self::Position) -> usize {
        let facing_score = match pos.dir {
            Vec2(1, 0, _) => 0,
            Vec2(0, 1, _) => 1,
            Vec2(-1, 0, _) => 2,
            Vec2(0, -1, _) => 3,
            _ => panic!("Invalid direction: {:?}", pos.dir),
        };
        ((pos.pos.1 + 1) * 1000 + (pos.pos.0 + 1) * 4) as usize + facing_score
    }
}
