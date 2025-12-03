#![warn(missing_debug_implementations)]

use std::{env, error, fs, process};

use aoc::{Day, SecretEntrance};

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
    let mut calendar = calendar();
    let index = (day as usize).min(1).max(calendar.len()) - 1;
    let puzzle = &mut calendar[index];

    println!("# Day {day}\n");

    let time = puzzle.measure_easy(&document).as_micros();
    let result = puzzle.format_easy().unwrap_or_default();
    println!("**Easy**\n  {result}\n  {time} μs\n");

    let time = puzzle.measure_hard(&document).as_micros();
    let result = puzzle.format_hard().unwrap_or_default();
    println!("**Hard**\n  {result}\n  {time} μs\n");
}

fn print_usage(name: &str) {
    eprintln!("usage: {name} <day> <document>");
}

fn print_error(error: &dyn error::Error) {
    eprintln!("error: {error}")
}

fn calendar() -> [Box<dyn Day>; 1] {
    [Box::new(SecretEntrance::new())]
}
