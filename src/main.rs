use aoc::Puzzle;

fn main() {
    println!("# Advent of Code 2025\n");

    for (i, location) in calendar().iter().enumerate() {
        let day = i + 1;
        println!("**Day {day}**\n");

        let (easy, hard) = *location;
        println!("Easy: {}", solve_and_format(easy));
        println!("Hard: {}", solve_and_format(hard));
    }
}

fn solve_and_format(puzzle: &dyn Puzzle) -> String {
    let (result, elapsed) = puzzle.measure();
    let micros = elapsed.as_micros();
    format!("{result}, {micros} μs")
}

const fn calendar() -> &'static [(&'static dyn Puzzle, &'static dyn Puzzle)] {
    &[]
}
