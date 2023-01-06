use crate::Day;

use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Day4(Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>);

#[inline]
fn parse_range(range: &str) -> RangeInclusive<usize> {
    let (start, end) = range.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

impl Day for Day4 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let (range_a, range_b) = line.split_once(',').unwrap();
                    (parse_range(range_a), parse_range(range_b))
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        self.0
            .iter()
            .filter(|(range_a, range_b)| {
                (range_a.start() <= range_b.start() && range_a.end() >= range_b.end())
                    || (range_b.start() <= range_a.start() && range_b.end() >= range_a.end())
            })
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .iter()
            .filter(|(range_a, range_b)| {
                (range_a.start() <= range_b.end()) && (range_a.end() >= range_b.start())
            })
            .count()
            .to_string()
    }
}
