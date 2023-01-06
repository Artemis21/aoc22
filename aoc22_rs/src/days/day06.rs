use crate::Day;

#[derive(Clone)]
pub struct Day6(&'static [u8]);

#[inline]
fn first_distinct(source: &[u8], n: usize) -> usize {
    let mut last_seen = [0; 26];
    let mut distinct = 0;
    for (i, c) in source.iter().enumerate() {
        let numerical = (c - b'a') as usize;
        if last_seen[numerical] + distinct < i {
            distinct += 1;
            if distinct == n {
                return i + 1;
            }
        } else {
            distinct = distinct.min(i - last_seen[numerical]);
        }
        last_seen[numerical] = i;
    }
    unreachable!()
}

impl Day for Day6 {
    fn parse(input: &'static str) -> Self {
        Self(input.as_bytes())
    }

    fn part1(&self) -> String {
        first_distinct(self.0, 4).to_string()
    }

    fn part2(&self) -> String {
        first_distinct(self.0, 14).to_string()
    }
}
