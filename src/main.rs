use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{env, usize};
use std::ops::{Index, IndexMut};

enum Role {
    Host,
    Visitor,
}

#[derive(Clone, Copy)]
struct AgentId(u32);

struct Network(Vec<Vec<f64>>);

impl Index<AgentId> for Network {
    type Output = Vec<f64>;
    
    fn index(&self, index: AgentId) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<AgentId> for Network {
    fn index_mut(&mut self, index: AgentId) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

struct Agent {
    agent_id: AgentId,
    strat_learning_speed: f64,
    net_learning_speed: f64,
    strat_discount: f64,
    net_discount: f64,
    strat_tremble: f64,
    net_tremble: f64,
    score: f64,
    current_partner: AgentId,
    strategy: Strategy,
    current_strategy: usize,
    current_payoff: usize,
}

struct Strategy {
    visit: [f64; 2],
    host: [f64; 2],
}

impl Agent {
    fn new(agent_id: AgentId, score: f64) -> Agent {
        Agent {
            agent_id,
            strat_learning_speed: 0.01,
            net_learning_speed: 0.01,
            strat_discount: 0.01,
            net_discount: 0.01,
            strat_tremble: 0.01,
            net_tremble: 0.01,
            score,
            current_partner: AgentId(0),
            strategy: Strategy::new(),
            current_strategy: 0,
            current_payoff: 0,
        }
    }

    fn partner_pick(
        &mut self,
        rng: &mut ChaCha8Rng,
        temp_vec: &[usize],
        network: &mut Network,
    ) -> AgentId {
        let mut friend_id: Option<usize> = None;
        let rand_tremble: f64 = rng.random();

        if rand_tremble < self.net_tremble {
            for i in 0..temp_vec.len() {
                network.discount_weight(self.agent_id, AgentId(i as u32), self.net_discount);
            }

            let partner_id: AgentId = AgentId(*temp_vec.choose(rng).unwrap() as u32);
            self.current_partner = partner_id;
            partner_id
        } else {
            let mut partial_sum: Vec<f64> = Vec::new();
            let mut acc: f64 = 0.0;

            for id in temp_vec {
                acc += network.0[self.agent_id.0 as usize][*id];
                partial_sum.push(acc);
            }

            let rand_prob: f64 = rng.random();
            let interaction_random_draw: f64 = rand_prob * partial_sum.last().unwrap_or(&0.0);

            for i in 0..partial_sum.len() {
                if friend_id.is_none() && interaction_random_draw <= partial_sum[i] {
                    friend_id = Some(temp_vec[i]);
                }

                network.discount_weight(self.agent_id, AgentId(i as u32), self.net_discount);
            }
            let partner_id: AgentId = AgentId(friend_id.unwrap_or_else(|| *temp_vec.choose(rng).unwrap()) as u32);
            self.current_partner = partner_id;
            partner_id
        }
    }

    fn choose_strategy(&mut self, rng: &mut ChaCha8Rng, role: Role) {
        let tremble_draw: f64 = rng.random();

        if tremble_draw < self.strat_tremble {
            self.current_strategy = rng.random_range(0..2);
            return;
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
            self.strategy
                .get_strat(&role)
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

    fn get_strat(&mut self, role: &Role) -> &mut [f64; 2] {
        match role {
            Role::Host => &mut self.host,
            Role::Visitor => &mut self.visit,
        }
    }
}

impl Network {
    fn discount_weight(&mut self, agent_id: AgentId, partner_id: AgentId, net_discount: f64) {
        self.0[agent_id.0 as usize][partner_id.0 as usize] *= 1.0 - net_discount;
    }
}

fn game() {}

fn run_time_step(
    rng: &mut ChaCha8Rng,
    agents: &mut Vec<Agent>,
    pop: usize,
    network: &mut Network,
) {
    let mut agent_seq: Vec<usize> = (0..pop).collect();
    agent_seq.shuffle(rng);

    for &id in &agent_seq {
        let mut temp_vec: Vec<usize> = agent_seq.iter().filter(|&&x| x != id).cloned().collect();

        let host_id: AgentId = agents[id].partner_pick(rng, &mut temp_vec, network);

        {
            let visitor = &mut agents[id];
            visitor.choose_strategy(rng, Role::Visitor);
            visitor.current_partner = host_id;
        }

        {
            let host = &mut agents[host_id.0 as usize];
            host.choose_strategy(rng, Role::Host);
        }
    }
}

fn main() {
    let seed = 0;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let max_time_step: usize = 100000000;
    let pop: usize = 20;

    let mut agents: Vec<Agent> = Vec::new();
    let mut network: Network = Network(vec![vec![0.5; pop]; pop]);

    for i in 0..pop {
        agents.push(Agent::new(AgentId(i as u32), rng.random()));
    }

    for i in 1..=max_time_step {
        run_time_step(&mut rng, &mut agents, pop, &mut network);
    }
}
