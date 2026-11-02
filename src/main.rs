mod environment;
mod graph;
mod q_learning;

use environment::{Environment, GridWorld};
use rand::RngExt;

use crate::q_learning::QLearning;
use graph::draw_rewards;

fn main() {
    let mut game = GridWorld::new();
    let mut agent = QLearning {
        q_table: [[0.0; 4]; 48],
        alpha: 0.1,
        gamma: 1.0,
    };
    let mut rng = rand::rng();
    let actions = 0..4;

    const EPISODES_NUM: usize = 10_000;
    const MAX_STEPS: usize = 1_000;

    let mut rewards = Vec::with_capacity(EPISODES_NUM);

    for episode in 0..EPISODES_NUM {
        let mut state = game.reset();
        let mut total_reward = 0.0;
        let epsilon = (1.0 - episode as f64 / EPISODES_NUM as f64).max(0.01);

        for _ in 0..MAX_STEPS {
            let action = if rng.random_bool(epsilon) {
                rng.random_range(actions.to_owned())
            } else {
                agent.best_action(state)
            };

            let result = game.step(action);

            agent.update(state, action, result.reward, result.next_state, result.done);

            state = result.next_state;
            total_reward += result.reward;

            if result.done {
                break;
            }
        }

        if (episode + 1) % 100 == 0 {
            println!("Episode: {}, Reward: {}", episode + 1, total_reward);
        }

        rewards.push(total_reward);
    }
    agent.print();

    let mut state = game.reset();

    loop {
        print!("{state} -> ");
        let action = agent.best_action(state);
        let result = game.step(action);

        state = result.next_state;

        if result.done {
            println!("{state} (GOAL)");
            break;
        }
    }

    draw_rewards(&rewards, 100).expect("Failed to draw graph");
}
