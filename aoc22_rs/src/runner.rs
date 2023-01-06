use crate::{cli::DayPick, data, timer, Day};
use log::{error, info};
use std::time::Duration;

pub fn run(data: data::All, picks: &[DayPick; 25]) -> Vec<DayTimings> {
    Runner { data, picks }.run()
}

macro_rules! run_days {
    ( $runner:expr, { $( $num:literal => $day_mod:ident::$day_type:ident, )* } ) => {
        [$( $runner.run_day($num, crate::days::$day_mod::$day_type::parse) ),*]
    };
}

struct Runner<'a> {
    data: data::All,
    picks: &'a [DayPick; 25],
}

impl<'a> Runner<'a> {
    fn run(&self) -> Vec<DayTimings> {
        run_days!(
            self,
            {
                1 => day01::Day1,
                2 => day02::Day2,
                3 => day03::Day3,
                4 => day04::Day4,
                5 => day05::Day5,
                6 => day06::Day6,
                7 => day07::Day7,
                8 => day08::Day8,
                9 => day09::Day9,
                10 => day10::Day10,
                11 => day11::Day11,
                12 => day12::Day12,
                13 => day13::Day13,
                14 => day14::Day14,
                15 => day15::Day15,
                16 => day16::Day16,
                17 => day17::Day17,
                18 => day18::Day18,
                19 => day19::Day19,
                20 => day20::Day20,
                21 => day21::Day21,
                22 => day22::Day22,
                23 => day23::Day23,
                24 => day24::Day24,
                25 => day25::Day25,
            }
        )
        .into_iter()
        .flatten()
        .collect()
    }

    fn run_day<D: Day>(
        &self,
        day_num: usize,
        parse_fn: fn(&'static str) -> D,
    ) -> Option<DayTimings> {
        let data = self.data[day_num - 1];
        let picks = self.picks[day_num - 1];
        if !(picks.parse || picks.part1 || picks.part2) {
            return None;
        }
        let (parse, day) = if picks.parse {
            let (timing, day) = timer::time(|| Ok::<_, ()>(parse_fn(data.input))).unwrap();
            report_parse(day_num, timing);
            (Some(timing), day)
        } else {
            (None, parse_fn(data.input))
        };
        let part1 = if picks.part1 {
            let result = timer::time(|| answer(day.part1(), data.part1)).map(|(d, ())| d);
            report_part(&format!("Day {day_num} part 1"), &result);
            Some(result)
        } else {
            None
        };
        let part2 = if picks.part2 {
            let result = timer::time(|| answer(day.part2(), data.part2)).map(|(d, ())| d);
            report_part(&format!("Day {day_num} part 2"), &result);
            Some(result)
        } else {
            None
        };
        Some(DayTimings {
            day: day_num as u32,
            parse,
            part1,
            part2,
        })
    }
}

pub struct DayTimings {
    pub day: u32,
    pub parse: Option<Duration>,
    pub part1: Option<Result<Duration, WrongAnswer>>,
    pub part2: Option<Result<Duration, WrongAnswer>>,
}

#[derive(Debug, Clone)]
pub struct WrongAnswer {
    pub expected: &'static str,
    pub actual: String,
}

fn answer(actual: String, expected: &'static str) -> Result<(), WrongAnswer> {
    if actual == expected {
        Ok(())
    } else {
        Err(WrongAnswer { expected, actual })
    }
}

fn report_parse(day: usize, timing: Duration) {
    info!("Day {} parsed in {:?}", day, timing);
}

fn report_part(part_name: &str, result: &Result<Duration, WrongAnswer>) {
    match result {
        Ok(timing) => info!("{part_name} gave correct answer in {timing:?}"),
        Err(WrongAnswer { expected, actual }) => {
            error!("{part_name} gave {actual} (expected {expected})");
        }
    }
}
