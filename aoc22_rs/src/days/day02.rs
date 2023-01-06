use crate::Day;

#[derive(Clone)]
pub struct Day2(Vec<(usize, usize)>);

#[inline]
const fn wins_against(opponent: usize) -> usize {
    (opponent + 1) % 3
}

#[inline]
const fn loses_against(opponent: usize) -> usize {
    (opponent + 2) % 3
}

impl Day for Day2 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let (them_raw, us_raw) = line.split_once(' ').unwrap();
                    ("ABC".find(them_raw).unwrap(), "XYZ".find(us_raw).unwrap())
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        self.0
            .iter()
            .map(|(them, us)| {
                if them == us {
                    4 + us
                } else if &wins_against(*them) == us {
                    7 + us
                } else {
                    1 + us
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .iter()
            .map(|(them, us)| match us {
                0 => 1 + loses_against(*them),
                1 => 4 + them,
                2 => 7 + wins_against(*them),
                _ => unreachable!(),
            })
            .sum::<usize>()
            .to_string()
    }
}
