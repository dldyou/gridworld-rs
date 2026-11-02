pub struct QLearning {
    pub q_table: [[f64; 4]; 48],
    pub alpha: f64,
    pub gamma: f64,
}

impl QLearning {
    pub fn update(
        &mut self,
        state: usize,
        action: usize,
        reward: f64,
        next_state: usize,
        done: bool,
    ) {
        let next_max = if done {
            0.0
        } else {
            self.q_table[next_state]
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
        };

        let current = self.q_table[state][action];
        let target = reward + self.gamma * next_max;

        self.q_table[state][action] += self.alpha * (target - current);
    }

    pub fn get_action(&self, state: usize) -> usize {
        let mut best = 0;

        for action in 1..4 {
            if self.q_table[state][action] > self.q_table[state][best] {
                best = action;
            }
        }

        best
    }

    pub fn print(&self) {
        let mut idx = 0;

        for row in self.q_table {
            let max_index = row
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .unwrap()
                .0;

            match max_index {
                0 => print!("⬆️"),
                1 => print!("⬇️"),
                2 => print!("⬅️"),
                3 => print!("➡️"),
                _ => unreachable!(),
            };
            print!(" ");

            idx += 1;

            if idx % 12 == 0 {
                println!("");
            }
        }
    }
}
