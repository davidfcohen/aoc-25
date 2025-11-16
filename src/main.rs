use aoc::Puzzle;

struct Location<'a> {
    easy: &'a dyn Puzzle,
    hard: &'a dyn Puzzle,
}

fn main() {
    println!("# Advent of Code 2025\n");

    if calendar().is_empty() {
        println!("The calendar is empty!");
    }

    for (i, location) in calendar().iter().enumerate() {
        let day = i + 1;
        println!("**Day {day}**");

        let easy = solve_and_format(location.easy);
        let hard = solve_and_format(location.hard);
        println!("  Easy: {easy}\n  Hard: {hard}\n");
    }
}

fn solve_and_format(puzzle: &dyn Puzzle) -> String {
    let (result, elapsed) = puzzle.measure();
    let micros = elapsed.as_micros();
    format!("{result}, {micros} μs")
}

const fn calendar() -> &'static [Location<'static>] {
    &[]
}
