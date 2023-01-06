use crate::vec2::Vec2;
use crate::Day;

use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Day9(Vec<Vec2>);

#[inline]
fn move_towards(vec: Vec2, target: Vec2) -> Vec2 {
    let delta = target - vec;
    if -1 <= delta.0 && delta.0 <= 1 && -1 <= delta.1 && delta.1 <= 1 {
        vec
    } else {
        vec + delta.signum()
    }
}

#[inline]
fn simulate_knot_movements<const N: usize>(movements: &[Vec2]) -> usize {
    let mut knots = [Vec2::new(0, 0); N];
    let mut seen = BTreeSet::new();
    let x_range = movements.len() as isize + 1;
    seen.insert(knots[0].as_base_n(x_range));
    for &direction in movements {
        knots[0] += direction;
        for i in 1..N {
            knots[i] = move_towards(knots[i], knots[i - 1]);
        }
        seen.insert(knots.last().unwrap().as_base_n(x_range));
    }
    seen.len()
}

impl Day for Day9 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .flat_map(|line| {
                    let (raw_direction, raw_distance) = line.split_once(' ').unwrap();
                    let direction = match raw_direction {
                        "U" => Vec2::new(0, 1),
                        "R" => Vec2::new(1, 0),
                        "D" => Vec2::new(0, -1),
                        "L" => Vec2::new(-1, 0),
                        _ => panic!("invalid direction"),
                    };
                    let distance = raw_distance.parse().unwrap();
                    vec![direction; distance]
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        simulate_knot_movements::<2>(&self.0).to_string()
    }

    fn part2(&self) -> String {
        simulate_knot_movements::<10>(&self.0).to_string()
    }
}
