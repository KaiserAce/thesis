use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{env, usize};

struct Agent {
    agent_id: usize,
    strat_learning_speed: f64,
    net_learning_speed: f64,
    strat_discount: f64,
    net_discount: f64,
    strat_tremble: f64,
    net_tremble: f64,
    score: f64,
    strategy: Strategy,
    current_strategy: usize,
}

struct Strategy {
    visit: [f64; 2],
    host: [f64; 2],
}

impl Agent {
    fn new(agent_id: usize, score: f64) -> Agent {
        Agent {
            agent_id,
            strat_learning_speed: 0.01,
            net_learning_speed: 0.01,
            strat_discount: 0.01,
            net_discount: 0.01,
            strat_tremble: 0.01,
            net_tremble: 0.01,
            score,
            strategy: Strategy::new(),
            current_strategy: 0,
        }
    }

    fn partner_pick(
        &mut self,
        rng: &mut ChaCha8Rng,
        temp_vec: &[usize],
        network: &mut Vec<Vec<f64>>,
    ) -> usize {
        let mut friend_id: Option<usize> = None;
        let rand_tremble: f64 = rng.random();

        if rand_tremble > self.net_tremble {
            let mut partial_sum: Vec<f64> = Vec::new();
            let mut acc: f64 = 0.0;

            for id in temp_vec {
                acc += network[self.agent_id][*id];
                partial_sum.push(acc);
            }

            let rand_prob: f64 = rng.random();
            let interaction_random_draw: f64 = rand_prob * partial_sum.last().unwrap_or(&0.0);

            for i in 0..partial_sum.len() {
                if friend_id.is_none() && interaction_random_draw <= partial_sum[i] {
                    friend_id = Some(temp_vec[i]);
                }

                network[self.agent_id][i] *= 1.0 - self.net_discount;
            }
            friend_id.unwrap_or_else(|| *temp_vec.choose(rng).unwrap())
        } else {
            for i in 0..temp_vec.len() {
                network[self.agent_id][i] *= 1.0 - self.net_discount;
            }

            *temp_vec.choose(rng).unwrap()
        }
    }

    fn choose_strategy(&mut self, rng: &mut ChaCha8Rng, role: String) {
        let tremble_draw: f64 = rng.random();

        if tremble_draw < self.strat_tremble {
            self.current_strategy = rng.random_range(0..2);
        } else {
            let strat_vec = self.strategy.get_strat(&role);

            let mut strat: Option<usize> = None;

            let partial_sum: Vec<f64> = strat_vec
                .iter()
                .scan(0.0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect();

            let rand_prob: f64 = rng.random();
            let weight_strat_draw = rand_prob * partial_sum.last().unwrap_or(&0.0);

            for i in 0..partial_sum.len() {
                if strat.is_none() && weight_strat_draw <= partial_sum[i] {
                    strat = Some(i);
                    self.current_strategy = strat.unwrap();
                }
            }
            self.strategy.get_strat(&role)
                .iter_mut()
                .for_each(|x| *x *= 1.0 - self.strat_discount);
        }
    }
}

impl Strategy {
    fn new() -> Strategy {
        Strategy {
            visit: [0.5, 0.5],
            host: [0.5, 0.5],
        }
    }

    fn get_strat(&mut self, role: &String) -> &mut [f64; 2] {
        if role == "host" {
            &mut self.host
        } else {
            &mut self.visit
        }
    }
}

fn game() {

}

fn run_time_step(
    rng: &mut ChaCha8Rng,
    agents: &mut Vec<Agent>,
    pop: usize,
    network: &mut Vec<Vec<f64>>,
) {
    let mut agent_seq: Vec<usize> = (0..pop).collect();
    agent_seq.shuffle(rng);

    for &id in &agent_seq {
        let mut temp_vec: Vec<usize> = agent_seq.iter().filter(|&&x| x != id).cloned().collect();

        let host_id: usize = agents[id].partner_pick(rng, &mut temp_vec, network);

        {
            let visitor = &mut agents[id];
            visitor.choose_strategy(rng, "visitor".to_string());
        }

        {
            let host = &mut agents[host_id];
            host.choose_strategy(rng, "host".to_string());
        }
    }
}

fn main() {
    let seed = 0;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let max_time_step: usize = 100000000;
    let pop: usize = 20;

    let mut agents: Vec<Agent> = Vec::new();
    let mut network = vec![vec![0.5; pop]; pop];

    for i in 0..pop {
        agents.push(Agent::new(i, rng.random()));
    }

    for i in 1..=max_time_step {
        run_time_step(&mut rng, &mut agents, pop, &mut network);
    }
}
