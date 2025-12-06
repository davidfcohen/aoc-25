#![warn(missing_debug_implementations)]

use std::{
    env, error, fs, process,
    time::{Duration, Instant},
};

use aoc::{GiftShop, Location, SecretEntrance};

fn main() {
    let mut args = env::args();
    let name = args.next().unwrap();

    let day = args.next().unwrap_or_else(|| {
        print_usage(&name);
        process::exit(1);
    });

    let document = args.next().unwrap_or_else(|| {
        print_usage(&name);
        process::exit(1);
    });

    let day = day.parse().unwrap_or_else(|error| {
        print_error(&error);
        process::exit(1);
    });

    let document = fs::read_to_string(document).unwrap_or_else(|error| {
        print_error(&error);
        process::exit(1);
    });

    run(day, &document)
}

fn run(day: u32, document: &str) {
    let calendar = calendar();
    let day = (day as usize).max(1).min(calendar.len());
    let location = calendar[day - 1];

    println!("# Day {day}\n");

    let (result, elapsed) = measure_easy(location, &document);
    println!("**Easy**\n  {result}\n  {} μs\n", elapsed.as_micros());

    let (result, elapsed) = measure_hard(location, &document);
    println!("**Hard**\n  {result}\n  {} μs\n", elapsed.as_micros());
}

fn print_usage(name: &str) {
    eprintln!("usage: {name} <day> <document>");
}

fn print_error(error: &dyn error::Error) {
    eprintln!("error: {error}")
}

fn measure_easy(location: &dyn Location, document: &str) -> (u64, Duration) {
    let start = Instant::now();
    let result = location.solve_easy(document);
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn measure_hard(location: &dyn Location, document: &str) -> (u64, Duration) {
    let start = Instant::now();
    let result = location.solve_hard(document);
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn calendar() -> [&'static dyn Location; 2] {
    [&SecretEntrance, &GiftShop]
}
