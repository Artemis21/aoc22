use rustc_hash::FxHashMap;

use crate::Day;

#[derive(Clone)]
pub struct Day20(Vec<isize>);

fn mix_score(nums: &[isize], iters: usize) -> isize {
    let len = nums.len() as isize - 1;
    let (coded, coding) = code_nums(nums);
    let mut positions: Vec<_> = coded.iter().map(|(_, start_pos)| *start_pos).collect();
    for _ in 0..iters {
        for &(num_mod, start_pos) in &coded {
            let idx_from = positions[start_pos];
            let mut idx_to = idx_from as isize + num_mod;
            if idx_to >= len {
                idx_to -= len;
            } else if idx_to < 0 {
                idx_to += len;
            }
            let idx_to = idx_to as usize;
            for i in &mut positions {
                if *i == idx_from {
                    *i = idx_to;
                } else if idx_from < idx_to && idx_from < *i && *i <= idx_to {
                    *i -= 1;
                } else if idx_to < idx_from && idx_to <= *i && *i < idx_from {
                    *i += 1;
                }
            }
        }
    }
    score_positions(&positions, &coding)
}

type Coded = (isize, usize);
type CodedNums = Vec<Coded>;
type Coding = FxHashMap<Coded, isize>;

fn code_nums(nums: &[isize]) -> (CodedNums, Coding) {
    let len = nums.len() as isize - 1;
    let mut coding = FxHashMap::default();
    let mut coded = Vec::new();
    for (counter, &num) in nums.iter().enumerate() {
        let coded_num = (num.rem_euclid(len), counter);
        coding.insert(coded_num, num);
        coded.push(coded_num);
    }
    (coded, coding)
}

fn score_positions(positions: &Vec<usize>, coding: &Coding) -> isize {
    let zero_start_pos = *coding.iter().find(|(_, &n)| n == 0).map(|((_num_mod, start_pos), _num)| start_pos).unwrap();
    let zero_idx = positions[zero_start_pos];
    let idx_a = (zero_idx + 1000) % positions.len();
    let idx_b = (zero_idx + 2000) % positions.len();
    let idx_c = (zero_idx + 3000) % positions.len();
    let mut total = 0;
    for (&(_num_mod, start_pos), num) in coding {
        let idx = positions[start_pos];
        if idx == idx_a || idx == idx_b || idx == idx_c {
            total += num;
        }
    }
    total
}

impl Day for Day20 {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(|l| l.parse().unwrap()).collect())
    }

    fn part1(&self) -> String {
        mix_score(&self.0, 1).to_string()
    }

    fn part2(&self) -> String {
        let big_nums = self.0.iter().map(|&n| n * 811_589_153).collect::<Vec<_>>();
        mix_score(&big_nums, 10).to_string()
    }
}
