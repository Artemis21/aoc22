mod cube_fold;
pub(self) mod parse;
mod part1;
mod part2;

use crate::Day;
pub(self) use cube_fold::{face_size, Cube, Face};
pub(self) use parse::{Instruction, Tile, Turn};
pub(self) use part1::{BasicMap, BasicPosition};
use part2::CubeMap;

#[derive(Clone)]
pub struct Day22 {
    map: BasicMap,
    instructions: Vec<Instruction>,
}

trait Map {
    type Position: Copy + std::fmt::Debug;

    fn next_position(&self, pos: Self::Position) -> Self::Position;
    fn turn_position(pos: Self::Position, turn: Turn) -> Self::Position;
    fn tile_at(&self, pos: Self::Position) -> Tile;
    fn start_point(&self) -> Self::Position;
    fn score_position(&self, pos: Self::Position) -> usize;

    fn follow_instruction(
        &self,
        mut pos: Self::Position,
        instruction: Instruction,
    ) -> Self::Position {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    let new_pos = self.next_position(pos);
                    if self.tile_at(new_pos) == Tile::Closed {
                        break;
                    }
                    pos = new_pos;
                }
                pos
            }
            Instruction::Turn(turn) => Self::turn_position(pos, turn),
        }
    }

    fn score_instructions(&self, instructions: &[Instruction]) -> usize {
        let mut pos = self.start_point();
        for &instruction in instructions {
            pos = self.follow_instruction(pos, instruction);
        }
        self.score_position(pos)
    }
}

impl Day for Day22 {
    fn parse(input: &str) -> Self {
        let (raw_map, raw_instructions) = input.split_once("\n\n").unwrap();
        let map = BasicMap::parse(raw_map);
        let instructions = parse::instructions(raw_instructions);
        Self { map, instructions }
    }

    fn part1(&self) -> String {
        self.map.score_instructions(&self.instructions).to_string()
    }

    fn part2(&self) -> String {
        CubeMap::from(self.map.clone())
            .score_instructions(&self.instructions)
            .to_string()
    }
}
