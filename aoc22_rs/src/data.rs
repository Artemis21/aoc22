use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub struct Day {
    pub day: u32,
    pub input: &'static str,
    pub part1: &'static str,
    pub part2: &'static str,
}

#[derive(Deserialize)]
struct Raw(&'static str, &'static str);

pub type All = Vec<Day>;

macro_rules! load_input {
    ($($day:literal,)*) => {
        [$(include_str!(concat!("inputs/day", $day, ".txt")),)*]
    };
}

pub fn load() -> All {
    let solutions: FxHashMap<usize, Raw> =
        serde_json::from_str(include_str!("solutions.json")).unwrap();
    let inputs = load_input![
        "01", "02", "03", "04", "05", "06", "07", "08", "09", 10, 11, 12, 13, 14, 15, 16, 17, 18,
        19, 20, 21, 22, 23, 24, 25,
    ];
    (1..=25)
        .into_iter()
        .map(|day| {
            let input = inputs[day - 1];
            let solution = solutions.get(&day).unwrap();
            Day {
                day: day as u32,
                input,
                part1: solution.0,
                part2: solution.1,
            }
        })
        .collect()
}
