pub trait Day {
    fn solve_easy(&mut self, input: impl AsRef<str>);
    fn solve_hard(&mut self, input: impl AsRef<str>);
}

pub mod secret_entrance {
    use super::Day;

    pub struct SecretEntrance {
        easy: Option<u32>,
        hard: Option<u32>,
    }

    impl Day for SecretEntrance {
        fn solve_easy(&mut self, input: impl AsRef<str>) {
            todo!()
        }

        fn solve_hard(&mut self, input: impl AsRef<str>) {
            todo!()
        }
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Right,
        Left,
    }

    fn parse_instruction(instruction: &[u8]) -> (Direction, u32) {
        let direction = instruction;
        todo!()
    }
}
