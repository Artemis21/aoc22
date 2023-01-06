use crate::vec2::{Vec2, CARDINALS};
use crate::Day;

#[derive(Clone)]
pub struct Day8(Vec<Vec<usize>>);

#[inline]
fn trees_visible(from: Vec2, facing: Vec2, map: &[Vec<usize>]) -> (usize, bool) {
    let mut pos = from;
    let mut trees = 0;
    let height = from.index(map);
    loop {
        pos += facing;
        if 0 > pos.0 || 0 > pos.1 || pos.0 >= map[0].len() as isize || pos.1 >= map.len() as isize {
            return (trees, false);
        }
        trees += 1;
        if pos.index(map) >= height {
            return (trees, true);
        }
    }
}

impl Day for Day8 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c as usize - '0' as usize).collect())
                .collect(),
        )
    }

    fn part1(&self) -> String {
        Vec2::map_coords(&self.0)
            .filter(|location| {
                CARDINALS
                    .iter()
                    .any(|facing| !trees_visible(*location, *facing, &self.0).1)
            })
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        Vec2::map_coords(&self.0)
            .map(|location| {
                CARDINALS
                    .iter()
                    .map(|facing| trees_visible(location, *facing, &self.0).0)
                    .product::<usize>()
            })
            .max()
            .unwrap()
            .to_string()
    }
}
