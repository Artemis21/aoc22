use crate::Day;

#[derive(Clone)]
pub struct Day3(Vec<Vec<usize>>);

impl Day for Day3 {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'a'..='z' => c as usize - 'a' as usize + 1,
                            'A'..='Z' => c as usize - 'A' as usize + 27,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn part1(&self) -> String {
        self.0
            .iter()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                *left.iter().find(|l| right.contains(l)).unwrap()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.0
            .chunks(3)
            .map(|chunk| {
                chunk[0]
                    .iter()
                    .find(|l| chunk[1].contains(l) && chunk[2].contains(l))
                    .unwrap()
            })
            .sum::<usize>()
            .to_string()
    }
}
