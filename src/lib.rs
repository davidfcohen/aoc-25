#![warn(missing_debug_implementations)]

use std::time::{Duration, Instant};

pub use gift_shop::GiftShop;
pub use secret_entrance::SecretEntrance;

pub trait Day {
    fn solve_easy(&mut self, _document: &str) {}

    fn solve_hard(&mut self, _document: &str) {}

    fn format_easy(&self) -> Option<String> {
        None
    }
    fn format_hard(&self) -> Option<String> {
        None
    }

    fn measure_easy(&mut self, document: &str) -> Duration {
        let start = Instant::now();
        self.solve_easy(document);
        start.elapsed()
    }

    fn measure_hard(&mut self, document: &str) -> Duration {
        let start = Instant::now();
        self.solve_hard(document);
        start.elapsed()
    }
}

pub mod secret_entrance {
    use super::Day;

    #[derive(Debug, Default, Clone)]
    pub struct SecretEntrance {
        easy: Option<u32>,
        hard: Option<u32>,
    }

    impl SecretEntrance {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Day for SecretEntrance {
        fn format_easy(&self) -> Option<String> {
            self.easy.map(|easy| format!("{easy}"))
        }

        fn format_hard(&self) -> Option<String> {
            self.hard.map(|hard| format!("{hard}"))
        }

        fn solve_easy(&mut self, document: &str) {
            let mut dial = Dial::new();

            let mut zeros = 0;
            for (direction, distance) in document.split('\n').filter_map(|i| parse_instruction(i)) {
                match direction {
                    Direction::Right => {
                        dial.rotate_right(distance);
                    }
                    Direction::Left => {
                        dial.rotate_left(distance);
                    }
                }

                if dial.tick() == 0 {
                    zeros += 1;
                }
            }

            self.easy = Some(zeros);
        }

        fn solve_hard(&mut self, document: &str) {
            let mut dial = Dial::new();

            let mut zeros = 0;
            for (direction, distance) in document.split('\n').filter_map(|i| parse_instruction(i)) {
                match direction {
                    Direction::Right => {
                        zeros += dial.rotate_right(distance);
                    }
                    Direction::Left => {
                        zeros += dial.rotate_left(distance);
                    }
                }
            }

            self.hard = Some(zeros);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Right,
        Left,
    }

    fn parse_instruction(instruction: &str) -> Option<(Direction, u32)> {
        let mut iter = instruction.as_bytes().iter();

        let direction = iter.next().and_then(|ch| match ch {
            b'R' => Some(Direction::Right),
            b'L' => Some(Direction::Left),
            _ => None,
        })?;

        let mut distance = 0;
        for ch in iter {
            if ch.is_ascii_digit() {
                distance = (distance * 10) + (ch - b'0') as u32;
            } else {
                return None;
            }
        }

        Some((direction, distance))
    }

    const DIAL_MAX: u32 = 99;
    const DIAL_MOD: u32 = DIAL_MAX + 1;
    const DIAL_START: u32 = 50;

    #[derive(Debug, Clone)]
    struct Dial {
        tick: u32,
    }

    impl Dial {
        fn new() -> Self {
            Self::default()
        }

        fn rotate_right(&mut self, distance: u32) -> u32 {
            let dist_from_zero = self.tick + distance;
            self.tick = dist_from_zero % DIAL_MOD;
            let clicks = dist_from_zero / (DIAL_MOD);
            clicks
        }

        fn rotate_left(&mut self, distance: u32) -> u32 {
            let was_tick_zero = self.tick == 0;
            let dist_from_max = (DIAL_MOD - self.tick) + distance;
            self.tick = (self.tick + DIAL_MOD - (distance % DIAL_MOD)) % DIAL_MOD;
            let zeros = dist_from_max / DIAL_MOD;
            if was_tick_zero { zeros - 1 } else { zeros }
        }

        fn tick(&self) -> u32 {
            self.tick
        }
    }

    impl Default for Dial {
        fn default() -> Self {
            Self { tick: DIAL_START }
        }
    }
}

pub mod gift_shop {
    use super::Day;

    #[derive(Debug, Default, Clone)]
    pub struct GiftShop {
        easy: Option<u64>,
        hard: Option<u64>,
    }

    impl GiftShop {
        pub fn new() -> Self {
            GiftShop::default()
        }
    }

    impl Day for GiftShop {
        fn format_easy(&self) -> Option<String> {
            self.easy.map(|easy| format!("{easy}"))
        }

        fn format_hard(&self) -> Option<String> {
            self.hard.map(|hard| format!("{hard}"))
        }

        fn solve_easy(&mut self, document: &str) {
            let mut invalid_sum = 0;

            for (start, end) in document.trim().split(',').filter_map(|r| parse_range(r)) {
                for product_id in start..=end {
                    let num_digits = product_id.ilog10() + 1;

                    if num_digits % 2 == 1 {
                        continue;
                    }

                    let first_half = product_id / 10u64.pow(num_digits / 2);
                    let second_half = product_id % 10u64.pow(num_digits / 2);

                    if first_half == second_half {
                        invalid_sum += product_id;
                        continue;
                    }
                }
            }

            self.easy = Some(invalid_sum);
        }

        fn solve_hard(&mut self, document: &str) {
            for (start, end) in document.trim().split(',').filter_map(|r| parse_range(r)) {
                for product_id in start..=end {
                    let num_digits = product_id.ilog10() + 1;

                    if num_digits % 2 == 1 {
                        continue;
                    }

                    // extract digits in window..
                }
            }
        }
    }

    fn parse_range(range: &str) -> Option<(u64, u64)> {
        let mut iter = range.split('-');
        let start = iter.next()?.parse().ok()?;
        let end = iter.next()?.parse().ok()?;
        Some((start, end))
    }

    fn extract_digits(product_id: u64, rpos: u32, len: u32) -> u64 {
        (product_id / 10u64.pow(rpos)) % 10u64.pow(len)
    }
}
