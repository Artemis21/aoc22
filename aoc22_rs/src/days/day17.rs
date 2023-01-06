use rustc_hash::{FxHashMap, FxHashSet};

use crate::{vec2::Vec2, Day};

#[derive(Clone)]
pub struct Day17(Vec<Jet>);

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    const fn as_delta(self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }
}

#[derive(Debug)]
struct Block {
    offsets: Vec<Vec2>,
    height: isize,
    width: isize,
    lower_edge: Vec<Vec2>,
    left_edge: Vec<Vec2>,
    right_edge: Vec<Vec2>,
}

impl Block {
    fn new(offsets: Vec<Vec2>) -> Self {
        let height = offsets.iter().map(|v| v.1).max().unwrap() + 1;
        let width = offsets.iter().map(|v| v.0).max().unwrap() + 1;
        let lower_edge = (0..width)
            .map(|x| {
                offsets
                    .iter()
                    .filter(|v| v.0 == x)
                    .min_by_key(|v| v.1)
                    .unwrap()
            })
            .copied()
            .collect();
        let left_edge = (0..height)
            .map(|y| {
                offsets
                    .iter()
                    .filter(|v| v.1 == y)
                    .min_by_key(|v| v.0)
                    .unwrap()
            })
            .copied()
            .collect();
        let right_edge = (0..height)
            .map(|y| {
                offsets
                    .iter()
                    .filter(|v| v.1 == y)
                    .max_by_key(|v| v.0)
                    .unwrap()
            })
            .copied()
            .collect();
        Self {
            offsets,
            height,
            width,
            lower_edge,
            left_edge,
            right_edge,
        }
    }

    fn from_raw(raw: &str) -> Self {
        let offsets = raw
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(x, y, _)| Vec2::new(x as isize, y as isize))
            .collect();
        Self::new(offsets)
    }
}

const RAW_BLOCKS: &str = "####

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
##";

lazy_static::lazy_static! {
    static ref BLOCKS: Vec<Block> = RAW_BLOCKS.split("\n\n").map(Block::from_raw).collect();
}

const WIDTH: isize = 7;
const DOWN: Vec2 = Vec2::new(0, -1);

struct Trench<'a> {
    jets: &'a [Jet],
    cells: FxHashSet<Vec2>,
    height: isize,
    buried_height: isize,
    jet_index: usize,
}

impl<'a> Trench<'a> {
    fn new(jets: &'a [Jet]) -> Self {
        Trench {
            jets,
            cells: FxHashSet::default(),
            height: 0,
            buried_height: 0,
            jet_index: 0,
        }
    }

    #[inline]
    fn next_jet(&mut self) -> Jet {
        let jet = self.jets.get(self.jet_index).copied().unwrap();
        self.jet_index += 1;
        if self.jet_index >= self.jets.len() {
            self.jet_index = 0;
        }
        jet
    }

    const fn full_height(&self) -> isize {
        self.height + self.buried_height
    }

    fn fall(&mut self, block: &Block) {
        let mut position = Vec2::new(2, self.height);
        let max_x = WIDTH - block.width;
        for _ in 0..4 {
            position += self.next_jet().as_delta();
            position.0 = position.0.clamp(0, max_x);
        }
        while position.1 > 0 && self.is_clear(position + DOWN, &block.lower_edge) {
            position += DOWN;
            let jet = self.next_jet();
            let edge = match jet {
                Jet::Left => &block.left_edge,
                Jet::Right => &block.right_edge,
            };
            let new_position = position + jet.as_delta();
            if self.is_clear(new_position, edge) {
                position = new_position;
                position.0 = position.0.clamp(0, max_x);
            }
        }
        for &offset in &block.offsets {
            self.cells.insert(position + offset);
        }
        self.height = self.height.max(block.height + position.1);
    }

    fn is_clear(&self, position: Vec2, offsets: &[Vec2]) -> bool {
        offsets
            .iter()
            .all(|&offset| !self.cells.contains(&(position + offset)))
    }

    fn clear_buried(&mut self) {
        let mut visited = FxHashSet::default();
        let mut open: Vec<Vec2> = Vec::new();
        for x in 0..WIDTH {
            open.push(Vec2::new(x, self.height - 1));
        }
        let mut cells = FxHashSet::default();
        while let Some(next) = open.pop() {
            if next.0 < 0 || next.0 >= WIDTH || next.1 < 0 || next.1 >= self.height {
                continue;
            }
            if !visited.insert(next) {
                continue;
            }
            if self.cells.contains(&next) {
                cells.insert(next);
            } else {
                open.push(next + Vec2::new(-1, 0));
                open.push(next + Vec2::new(1, 0));
                open.push(next + Vec2::new(0, -1));
            }
        }
        if cells.is_empty() {
            return;
        }
        let height_buried = cells.iter().map(|v| v.1).min().unwrap();
        self.cells = cells
            .iter()
            .map(|&v| v - Vec2::new(0, height_buried))
            .collect();
        self.buried_height += height_buried;
        self.height -= height_buried;
    }
}

impl Day for Day17 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .chars()
                .map(|c| match c {
                    '<' => Jet::Left,
                    '>' => Jet::Right,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        let mut trench = Trench::new(&self.0);
        for block in BLOCKS.iter().cycle().take(2022) {
            trench.fall(block);
        }
        trench.full_height().to_string()
    }

    fn part2(&self) -> String {
        let mut trench = Trench::new(&self.0);
        let mut visited = FxHashMap::default();
        let mut remaining = 1_000_000_000_000;
        while remaining >= BLOCKS.len() as isize {
            for block in BLOCKS.iter() {
                trench.fall(block);
            }
            remaining -= BLOCKS.len() as isize;
            //    if trench.jet_index == 0 {
            trench.clear_buried();
            let mut key: Vec<_> = trench.cells.iter().copied().collect();
            key.sort();
            if let Some((remaining_then, height_then)) =
                visited.insert((trench.jet_index, key), (remaining, trench.full_height()))
            {
                let period = remaining_then - remaining;
                let increment = trench.full_height() - height_then;
                let cycles = remaining / period;
                remaining %= period;
                trench.buried_height += cycles * increment;
                break;
                //               }
            }
        }
        for block in BLOCKS.iter().cycle().take(remaining as usize) {
            trench.fall(block);
        }
        trench.full_height().to_string()
    }
}
