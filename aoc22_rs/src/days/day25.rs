use crate::Day;

fn snafu_digit_to_int(digit: u8) -> isize {
    match digit {
        b'=' => -2,
        b'-' => -1,
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        _ => panic!("invalid digit"),
    }
}

fn snafu_to_int(snafu: &str) -> isize {
    snafu
        .as_bytes()
        .iter()
        .copied()
        .map(snafu_digit_to_int)
        .fold(0, |acc, x| acc * 5 + x)
}

fn int_to_snafu_digit(digit: isize) -> char {
    match digit {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("invalid digit"),
    }
}

fn int_to_snafu(mut n: isize) -> String {
    let mut snafu = String::new();
    while n != 0 {
        let new = (n + 2) / 5;
        let digit = n - new * 5;
        snafu.push(int_to_snafu_digit(digit));
        n = new;
    }
    snafu.chars().rev().collect()
}

#[derive(Clone)]
pub struct Day25(Vec<isize>);

impl Day for Day25 {
    fn parse(input: &'static str) -> Self {
        Self(input.lines().map(snafu_to_int).collect())
    }

    fn part1(&self) -> String {
        int_to_snafu(self.0.iter().sum())
    }

    fn part2(&self) -> String {
        String::new()
    }
}
