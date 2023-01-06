use std::str::FromStr;

use clap::Parser;

/// Run Advent of Code 2022 solutions with testing and timing.
#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Which parts to run.
    ///
    /// Examples:
    ///
    /// - `3`: Run both parts of day 3 including timing parsing
    /// - `14a`: Run the first part of day 14
    /// - `6b`: Run the second part of day 6
    /// - `23p`: Run only the parsing of day 23
    /// - `9a 9b 10p`: Run both parts of day 9 (but don't time parsing), and only time parsing for day 10
    /// - `all`: Run all parts of all days
    #[arg(default_value = "all", verbatim_doc_comment)]
    parts: Vec<PartSelection>,
}

impl Args {
    pub fn get_picks() -> [DayPick; 25] {
        let args = Self::parse();
        pretty_env_logger::formatted_builder()
            .filter_level(args.verbose.log_level_filter())
            .init();
        normalise_parts(&args.parts)
    }
}

#[derive(Clone, Debug, Copy)]
enum PartSelection {
    All,
    Day(usize),
    Parse(usize),
    Part1(usize),
    Part2(usize),
}

impl FromStr for PartSelection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "all" {
            Ok(Self::All)
        } else if let Some(day) = s.strip_suffix('a') {
            day.parse().map(PartSelection::Part1).map_err(|_| "invalid day")
        } else if let Some(day) = s.strip_suffix('b') {
            day.parse().map(PartSelection::Part2).map_err(|_| "invalid day")
        } else if let Some(day) = s.strip_suffix('p') {
            day.parse().map(PartSelection::Parse).map_err(|_| "invalid day")
        } else {
            s.parse().map(PartSelection::Day).map_err(|_| "invalid day")
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct DayPick {
    pub day: usize,
    pub parse: bool,
    pub part1: bool,
    pub part2: bool,
}

fn normalise_parts(parts: &[PartSelection]) -> [DayPick; 25] {
    let mut days = std::array::from_fn(|day| DayPick {
        day: day + 1,
        parse: false,
        part1: false,
        part2: false,
    });
    for part in parts {
        match part {
            PartSelection::All => {
                return std::array::from_fn(|day| DayPick {
                    day: day + 1,
                    parse: true,
                    part1: true,
                    part2: true,
                });
            }
            PartSelection::Day(day) => {
                days[day - 1].parse = true;
                days[day - 1].part1 = true;
                days[day - 1].part2 = true;
            }
            PartSelection::Parse(day) => days[day - 1].parse = true,
            PartSelection::Part1(day) => days[day - 1].part1 = true,
            PartSelection::Part2(day) => days[day - 1].part2 = true,
        }
    }
    days
}
