use rustc_hash::FxHashSet;

use crate::{vec2::Vec2, Day};

#[derive(Clone)]
pub struct Day14 {
    map: FxHashSet<Vec2>,
    min_x: isize,
    max_x: isize,
    max_y: isize,
}

const SPAWN: Vec2 = Vec2::new(500, 0);
const FALLS: [Vec2; 3] = [Vec2::new(0, 1), Vec2::new(-1, 1), Vec2::new(1, 1)];

impl Day14 {
    fn fall_sand(&mut self, from: Vec2, hard_bottom: bool) -> Option<Vec<Vec2>> {
        if self.map.get(&from).is_some() {
            return None;
        }
        let mut point = from;
        let mut trace = Vec::new();
        'one_fall: while (self.min_x..=self.max_x).contains(&point.0) && point.1 < self.max_y {
            if self.map.get(&point).is_some() {
                break;
            }
            for &fall in &FALLS {
                let new_point = point + fall;
                if self.map.get(&new_point).is_none() {
                    trace.push(point);
                    point = new_point;
                    continue 'one_fall;
                }
            }
            self.map.insert(point);
            return Some(trace);
        }
        if hard_bottom && point.1 >= self.max_y {
            self.map.insert(point);
            return Some(trace);
        }
        None
    }

    fn count_settle(&self, hard_bottom: bool) -> usize {
        let mut map = self.clone();
        let mut total = 0;
        let mut stack = vec![SPAWN];
        while let Some(from) = stack.pop() {
            let Some(trace) = map.fall_sand(from, hard_bottom) else {
                continue;
            };
            total += 1;
            stack.extend(trace);
        }
        total
    }
}

impl Day for Day14 {
    fn parse(input: &str) -> Self {
        let mut map = FxHashSet::default();
        let (mut min_x, mut max_x, mut max_y) = (isize::MAX, isize::MIN, 0);
        for line in input.lines() {
            let mut last_point: Option<Vec2> = None;
            for point in line.split(" -> ") {
                let (x, y) = point.split_once(',').unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                let point = Vec2::new(x, y);
                if let Some(last_point) = last_point {
                    for point in last_point.range_inclusive(point) {
                        map.insert(point);
                    }
                }
                last_point = Some(point);
            }
        }
        max_y += 1;
        Self {
            map,
            min_x: min_x - max_y,
            max_x: max_x + max_y,
            max_y,
        }
    }

    fn part1(&self) -> String {
        self.count_settle(false).to_string()
    }

    fn part2(&self) -> String {
        self.count_settle(true).to_string()
    }
}
