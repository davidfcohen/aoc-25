#![warn(missing_debug_implementations)]

pub use gift_shop::GiftShop;
pub use secret_entrance::SecretEntrance;

pub trait Location {
    fn solve_easy(&self, document: &str) -> u64;
    fn solve_hard(&self, document: &str) -> u64;
}

pub mod secret_entrance {
    use super::Location;

    #[derive(Debug, Clone)]
    pub struct SecretEntrance;

    impl Location for SecretEntrance {
        fn solve_easy(&self, document: &str) -> u64 {
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

            zeros as u64
        }

        fn solve_hard(&self, document: &str) -> u64 {
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

            zeros as u64
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
    use super::Location;

    #[derive(Debug, Clone)]
    pub struct GiftShop;

    impl Location for GiftShop {
        fn solve_easy(&self, document: &str) -> u64 {
            let mut invalid_sum = 0;

            for (start, end) in document
                .trim()
                .split(',')
                .filter_map(|range| parse_range(range))
            {
                for product_id in start..=end {
                    let len = count_digits(product_id);

                    if len % 2 != 0 {
                        continue;
                    }

                    let mid = len / 2;
                    let left = rslice_digits(product_id, mid, mid);
                    let right = rslice_digits(product_id, 0, mid);

                    if left == right {
                        invalid_sum += product_id;
                    }
                }
            }

            invalid_sum
        }

        fn solve_hard(&self, document: &str) -> u64 {
            let mut invalid_sum = 0;

            for (start, end) in document
                .trim()
                .split(',')
                .filter_map(|range| parse_range(range))
            {
                for product_id in start..=end {
                    if is_repeated_pattern(product_id) {
                        invalid_sum += product_id;
                    }
                }
            }

            invalid_sum
        }
    }

    fn is_repeated_pattern(product_id: u64) -> bool {
        let len = count_digits(product_id);

        let mid = len / 2;
        (1..=mid)
            .rev()
            .any(|window| is_repeated_pattern_window(product_id, len, window))
    }

    fn is_repeated_pattern_window(product_id: u64, len: u32, window: u32) -> bool {
        if len % window != 0 {
            return false;
        }

        let mut right = rslice_digits(product_id, 0, window);
        (window..len).step_by(window as usize).all(|left_rpos| {
            let left = rslice_digits(product_id, left_rpos, window);
            let is_eq = left == right;
            right = left;
            is_eq
        })
    }

    fn parse_range(range: &str) -> Option<(u64, u64)> {
        let mut iter = range.split('-');
        let start = iter.next()?.parse().ok()?;
        let end = iter.next()?.parse().ok()?;
        Some((start, end))
    }

    fn count_digits(product_id: u64) -> u32 {
        product_id.ilog10() + 1
    }

    fn rslice_digits(product_id: u64, rpos: u32, len: u32) -> u64 {
        assert!(len <= product_id.ilog10() + 1);
        (product_id / 10u64.pow(rpos)) % 10u64.pow(len)
    }
}
