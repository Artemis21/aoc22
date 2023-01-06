#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    // missing_docs,
    // clippy::missing_docs_in_private_items
)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss
)]
#![feature(const_option)]
#![feature(const_for)]

mod cli;
mod data;
mod days;
mod output;
mod runner;
mod timer;
mod util;
mod vec2;

trait Day {
    fn parse(input: &'static str) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn main() {
    let picks = cli::Args::get_picks();
    let data = data::load();
    let timings = runner::run(data, &picks);
    output::print_timings(&timings);
}
