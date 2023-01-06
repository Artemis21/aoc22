use std::collections::BinaryHeap;

use rustc_hash::FxHashSet;

use crate::{Day, vec2::{Vec2, CARDINALS}};

#[derive(Clone)]
pub struct Day24(Valley);

#[derive(Clone)]
struct Valley {
    generations: Vec<Map>,
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
}

#[derive(Clone)]
struct Map(Vec<Vec<bool>>);

impl Map {
    fn occupied(&self, position: Vec2) -> bool {
        let max_x = self.0[0].len() as isize - 1;
        let max_y = self.0.len() as isize - 1;
        if position.0 < 0 || position.0 > max_x {
            true
        } else if position == Vec2::new(0, -1) || position == Vec2::new(max_x, max_y + 1) {
            false
        } else if position.1 < 0 || position.1 > max_y {
            true
        } else {
            self.0[position.1 as usize][position.0 as usize]
        }
    }
}

#[derive(Clone, Copy)]
struct Blizzard {
    position: Vec2,
    direction: Vec2,
}

impl Blizzard {
    fn advance(&mut self, width: usize, height: usize) {
        let (max_x, max_y) = (width as isize - 1, height as isize - 1);
        self.position += self.direction;
        if self.position.0 < 0 {
            self.position.0 = max_x;
        } else if self.position.0 > max_x {
            self.position.0 = 0;
        } else if self.position.1 < 0 {
            self.position.1 = max_y;
        } else if self.position.1 > max_y {
            self.position.1 = 0;
        }
    }
}

impl Valley {
    fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap() - 2;
        let height = input.lines().count() - 2;
        let blizzards = input.lines().enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x as isize - 1, y as isize - 1, c)))
            .flat_map(|(x, y, c)| match c {
                '^' => Some(Blizzard { position: Vec2::new(x, y), direction: Vec2::new(0, -1) }),
                'v' => Some(Blizzard { position: Vec2::new(x, y), direction: Vec2::new(0, 1) }),
                '<' => Some(Blizzard { position: Vec2::new(x, y), direction: Vec2::new(-1, 0) }),
                '>' => Some(Blizzard { position: Vec2::new(x, y), direction: Vec2::new(1, 0) }),
                _ => None,
            })
            .collect();
        let generations = Vec::new();
        Self { generations, width, height, blizzards }
    }

    fn generation(&mut self, index: usize) -> &Map {
        while index >= self.generations.len() {
            self.simulate_generation();
        }
        &self.generations[index]
    }

    fn simulate_generation(&mut self) {
        let mut map = Map(vec![vec![false; self.width]; self.height]);
        for blizzard in &mut self.blizzards {
            *blizzard.position.index_mut(&mut map.0) = true;
            blizzard.advance(self.width, self.height);
        }
        self.generations.push(map);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    position: Vec2,
    steps: usize,
    target: Vec2,
}

impl State {
    fn lower_bound(&self) -> usize {
        self.steps + (self.target - self.position).manhattan()
    }

    fn children(&self, valley: &mut Valley) -> Vec<Self> {
        let steps = self.steps + 1;
        let map = valley.generation(steps);
        CARDINALS
            .iter()
            .chain(std::iter::once(&Vec2::new(0, 0)))
            .flat_map(move |&direction| {
                let position = self.position + direction;
                if map.occupied(position) {
                    None
                } else {
                    Some(State { position, steps, ..*self })
                }
            })
            .collect()
    }

    fn best_path(self, valley: &mut Valley) -> Option<Self> {
        let mut queue = BinaryHeap::new();
        queue.push(self);
        let mut visited = FxHashSet::default();
        while let Some(state) = queue.pop() {
            if state.position == state.target {
                return Some(state);
            }
            for child in state.children(valley) {
                if visited.insert(child) {
                    queue.push(child);
                }
            }
        }
        None
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Inverted for max heap
        other.lower_bound().cmp(&self.lower_bound())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day for Day24 {
    fn parse(input: &str) -> Self {
        Self(Valley::parse(input))
    }

    fn part1(&self) -> String {
        let mut valley = self.0.clone();
        State {
            position: Vec2::new(0, -1),
            steps: 0,
            target: Vec2::new(valley.width as isize - 1, valley.height as isize),
        }.best_path(&mut valley).unwrap().steps.to_string()
    }

    fn part2(&self) -> String {
        let mut valley = self.0.clone();
        let mut state = State {
            position: Vec2::new(0, -1),
            steps: 0,
            target: Vec2::new(valley.width as isize - 1, valley.height as isize),
        }.best_path(&mut valley).unwrap();
        state = State {
            target: Vec2::new(0, -1),
            ..state
        }.best_path(&mut valley).unwrap();
        State {
            target: Vec2::new(valley.width as isize - 1, valley.height as isize),
            ..state
        }.best_path(&mut valley).unwrap().steps.to_string()
    }
}
