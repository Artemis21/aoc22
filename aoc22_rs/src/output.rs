use crate::runner::{DayTimings, WrongAnswer};
use std::time::Duration;

const DAY_WIDTH: usize = 5;
const TIMING_WIDTH: usize = 15;

pub fn print_timings(timings: &[DayTimings]) {
    Row::print_top();
    Row {
        day: "Day".into(),
        parse: "Parse".into(),
        part1: "Part 1".into(),
        part2: "Part 2".into(),
        total: "Total".into(),
    }
    .print();
    Row::print_sep();
    let mut parse_total = Duration::ZERO;
    let mut part1_total = Duration::ZERO;
    let mut part2_total = Duration::ZERO;
    for timing in timings {
        let parse_time = timing
            .parse
            .as_ref()
            .map_or(Duration::ZERO, |timing| *timing);
        let part1_time = match &timing.part1 {
            Some(Ok(timing)) => *timing,
            _ => Duration::ZERO,
        };
        let part2_time = match &timing.part2 {
            Some(Ok(timing)) => *timing,
            _ => Duration::ZERO,
        };
        parse_total += parse_time;
        part1_total += part1_time;
        part2_total += part2_time;
        let total = parse_time + part1_time + part2_time;
        Row {
            day: timing.day.to_string().into(),
            parse: format_timing(&timing.parse.map(Ok)),
            part1: format_timing(&timing.part1),
            part2: format_timing(&timing.part2),
            total: format!("{total:?}").into(),
        }
        .print();
    }
    Row::print_sep();
    Row {
        day: "Total".into(),
        parse: format!("{parse_total:?}").into(),
        part1: format!("{part1_total:?}").into(),
        part2: format!("{part2_total:?}").into(),
        total: format!("{:?}", parse_total + part1_total + part2_total).into(),
    }
    .print();
    Row::print_bottom();
}

fn format_timing(timing: &Option<Result<Duration, WrongAnswer>>) -> Cell {
    match timing {
        Some(Ok(timing)) => Cell::from(format!("{timing:?}")).fg(timing_colour(*timing)),
        Some(Err(_)) => Cell::from("Wrong!")
            .bg(Colour { r: 255, g: 0, b: 0 })
            .fg(Colour { r: 0, g: 0, b: 0 }),
        None => Cell::from("-"),
    }
}

struct Cell {
    text: String,
    fg: Option<Colour>,
    bg: Option<Colour>,
}

struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Cell {
    fn display(&self, width: usize) -> String {
        let mut text = format!("{:^width$}", self.text);
        if let Some(Colour { r, g, b }) = self.fg {
            text = format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m");
        };
        if let Some(Colour { r, g, b }) = self.bg {
            text = format!("\x1b[48;2;{r};{g};{b}m{text}\x1b[0m");
        };
        text
    }

    const fn fg(mut self, colour: Colour) -> Self {
        self.fg = Some(colour);
        self
    }

    const fn bg(mut self, colour: Colour) -> Self {
        self.bg = Some(colour);
        self
    }
}

struct Row {
    day: Cell,
    parse: Cell,
    part1: Cell,
    part2: Cell,
    total: Cell,
}

impl Row {
    fn print(&self) {
        println!(
            "│ {} │ {} │ {} │ {} │ {} │",
            self.day.display(DAY_WIDTH),
            self.parse.display(TIMING_WIDTH),
            self.part1.display(TIMING_WIDTH),
            self.part2.display(TIMING_WIDTH),
            self.total.display(TIMING_WIDTH)
        );
    }

    fn print_top() {
        let day_sep = "─".repeat(DAY_WIDTH + 2);
        let timing_sep = "─".repeat(TIMING_WIDTH + 2);
        println!("╭{day_sep}┬{timing_sep}┬{timing_sep}┬{timing_sep}┬{timing_sep}╮");
    }

    fn print_bottom() {
        let day_sep = "─".repeat(DAY_WIDTH + 2);
        let timing_sep = "─".repeat(TIMING_WIDTH + 2);
        println!("╰{day_sep}┴{timing_sep}┴{timing_sep}┴{timing_sep}┴{timing_sep}╯");
    }

    fn print_sep() {
        let day_sep = "─".repeat(DAY_WIDTH + 2);
        let timing_sep = "─".repeat(TIMING_WIDTH + 2);
        println!("├{day_sep}┼{timing_sep}┼{timing_sep}┼{timing_sep}┼{timing_sep}┤");
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Self {
            text,
            fg: None,
            bg: None,
        }
    }
}

impl<'a> From<&'a str> for Cell {
    fn from(text: &'a str) -> Self {
        Self {
            text: text.to_string(),
            fg: None,
            bg: None,
        }
    }
}

fn timing_colour(timing: Duration) -> Colour {
    let from = ((timing.as_micros() as f64).log10() / 8.0).clamp(0.0, 1.0);
    if from < 0.5 {
        Colour {
            r: (255.0 * from * 2.0) as u8,
            g: 255,
            b: 0,
        }
    } else {
        Colour {
            r: 255,
            g: (255.0 * (1.0 - from) * 2.0) as u8,
            b: 0,
        }
    }
}
