pub trait Environment {
    type State: Copy;
    type Action: Copy;

    fn reset(&mut self) -> Self::State;

    fn step(&mut self, action: Self::Action) -> Transition<Self::State>;
}

#[derive(Debug)]
pub struct Transition<State> {
    pub next_state: State,
    pub reward: f64,
    pub done: bool,
}

pub struct GridWorld {
    position: usize,
    done: bool,
}

impl GridWorld {
    pub const WIDTH: usize = 12;
    pub const HEIGHT: usize = 4;

    pub const START: usize = 3 * 12 + 0;
    pub const GOAL: usize = 3 * 12 + 11;

    pub fn new() -> Self {
        Self {
            position: Self::START,
            done: false,
        }
    }

    fn is_cliff(position: usize) -> bool {
        (37..47).contains(&position)
    }
}

impl Environment for GridWorld {
    type State = usize;
    type Action = usize;

    fn reset(&mut self) -> Self::State {
        self.position = Self::START;
        self.done = false;

        self.position
    }

    fn step(&mut self, action: Self::Action) -> Transition<Self::State> {
        if self.done {
            return Transition {
                next_state: self.position,
                reward: 0.0,
                done: true,
            };
        }

        let x = self.position % Self::WIDTH;
        let y = self.position / Self::WIDTH;

        let (next_x, next_y) = match action {
            0 => (x, y.saturating_sub(1)),
            1 => (x, (y + 1).min(Self::HEIGHT - 1)),
            2 => (x.saturating_sub(1), y),
            3 => ((x + 1).min(Self::WIDTH - 1), y),
            _ => unreachable!("unexpected action (only 0..4)"),
        };

        let next_position = next_y * Self::WIDTH + next_x;

        if Self::is_cliff(next_position) {
            self.position = Self::START;

            return Transition {
                next_state: self.position,
                reward: -100.0,
                done: false,
            };
        }

        self.position = next_position;
        self.done = self.position == Self::GOAL;

        Transition {
            next_state: self.position,
            reward: -1.0,
            done: self.done,
        }
    }
}

impl Default for GridWorld {
    fn default() -> Self {
        Self::new()
    }
}
