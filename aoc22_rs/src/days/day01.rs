use crate::util::max_n;
use crate::Day;

#[derive(Clone)]
pub struct Day1(Vec<usize>);

impl Day for Day1 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .split("\n\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|line| line.parse::<usize>().unwrap())
                        .sum()
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        self.0.iter().max().unwrap().to_string()
    }

    fn part2(&self) -> String {
        max_n::<3, _>(&self.0)
            .into_iter()
            .sum::<usize>()
            .to_string()
    }
}
