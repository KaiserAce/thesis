use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::ops::{Index, IndexMut};
use std::{env, usize};

enum Role {
    Host,
    Visitor,
}

#[derive(Clone, Copy)]
enum Strategy {
    None,
    Hawk,
    Dove,
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

impl Index<AgentId> for Vec<Agent> {
    type Output = Agent;
    fn index(&self, index: AgentId) -> &Agent {
        &self[index.0 as usize]
    }
}

impl IndexMut<AgentId> for Vec<Agent> {
    fn index_mut(&mut self, index: AgentId) -> &mut Self::Output {
        &mut self[index.0 as usize]
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
    total_payoff: f64,
    current_partner: AgentId,
    strategy: StratVector,
    current_strategy: Strategy,
    current_payoff: f64,
}

struct StratVector {
    visit: [f64; 2],
    host: [f64; 2],
}

struct PayoffMap {
    host: Vec<Vec<f32>>,
    visitor: Vec<Vec<f32>>,
    win: f32,
    lose: f32,
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
            total_payoff: 0.0,
            current_partner: AgentId(0),
            strategy: StratVector::new(),
            current_strategy: Strategy::None,
            current_payoff: 0.0,
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
            let partner_id: AgentId =
                AgentId(friend_id.unwrap_or_else(|| *temp_vec.choose(rng).unwrap()) as u32);
            self.current_partner = partner_id;
            partner_id
        }
    }

    fn choose_strategy(&mut self, rng: &mut ChaCha8Rng, role: Role) {
        let tremble_draw: f64 = rng.random();

        if tremble_draw < self.strat_tremble {
            let strategy = rng.random_range(0..2);
            self.current_strategy = match strategy {
                0 => Strategy::Hawk,
                1 => Strategy::Dove,
                _ => unreachable!(),
            };
            let strat_vec = self.strategy.get_strat(&role);
            strat_vec
                .iter_mut()
                .for_each(|x| *x *= 1.0 - self.strat_discount);
            return;
        } else {
            let strat_vec = self.strategy.get_strat(&role);

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
                if weight_strat_draw <= partial_sum[i] {
                    let strategy = i;
                    self.current_strategy = match strategy {
                        0 => Strategy::Hawk,
                        1 => Strategy::Dove,
                        _ => unreachable!(),
                    };
                    break;
                }
            }
            let strat_vec = self.strategy.get_strat(&role);
            strat_vec
                .iter_mut()
                .for_each(|x| *x *= 1.0 - self.strat_discount);
        }
    }

    fn add_network_payoff(&mut self, network: &mut Network) {
        network[self.agent_id][self.current_partner.0 as usize] += self.current_payoff * self.net_learning_speed;
    }

    fn add_strategy_payoff(&mut self, role: Role) {
        let index = match self.current_strategy {
            Strategy::Hawk => 0,
            Strategy::Dove => 1,
            _ => unreachable!()
        };

        match role {
            Role::Visitor => self.strategy.visit[index] += self.current_payoff * self.strat_learning_speed,
            Role::Host => self.strategy.host[index] += self.current_payoff * self.strat_learning_speed,
        }
        
        self.total_payoff += self.current_payoff
    }
}

impl StratVector {
    fn new() -> StratVector {
        StratVector {
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

impl PayoffMap {
    fn new() -> PayoffMap {
        PayoffMap {
            host: vec![[0.0, 0.4].to_vec(), [1.0, 0.6].to_vec()],
            visitor: vec![[0.0, 1.0].to_vec(), [0.4, 0.6].to_vec()],
            win: 0.6,
            lose: 0.2,
        }
    }
}

fn game(
    _rng: &mut ChaCha8Rng,
    visitor: AgentId,
    host: AgentId,
    agents: &mut Vec<Agent>,
    payoffs: &PayoffMap,
) {
    let visitor_score: f64 = agents[visitor].score;
    let host_score: f64 = agents[host].score;

    let visitor_strategy: Strategy = agents[visitor].current_strategy;
    let host_strategy: Strategy = agents[host].current_strategy;

    match (visitor_strategy, host_strategy) {
        (Strategy::Hawk, Strategy::Hawk) => {
            if visitor_score > host_score {
                agents[visitor].current_payoff = payoffs.win as f64;
                agents[host].current_payoff = payoffs.lose as f64;
            } else {
                agents[visitor].current_payoff = payoffs.lose as f64;
                agents[host].current_payoff = payoffs.win as f64;
            }
        }
        (Strategy::Hawk, Strategy::Dove) => {
            agents[visitor].current_payoff = payoffs.visitor[0][1] as f64;
            agents[host].current_payoff = payoffs.host[0][1] as f64;
        }
        (Strategy::Dove, Strategy::Hawk) => {
            agents[visitor].current_payoff = payoffs.visitor[1][0] as f64;
            agents[host].current_payoff = payoffs.host[1][0] as f64;
        }
        (Strategy::Dove, Strategy::Dove) => {
            agents[visitor].current_payoff = payoffs.visitor[1][1] as f64;
            agents[host].current_payoff = payoffs.host[1][1] as f64;
        }
        _ => unreachable!(),
    }
}

fn run_time_step(
    rng: &mut ChaCha8Rng,
    agents: &mut Vec<Agent>,
    pop: usize,
    network: &mut Network,
    payoffs: &PayoffMap,
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

        game(rng, AgentId(id as u32), host_id, agents, payoffs);

        agents[id].add_network_payoff(network);

        agents[id].add_strategy_payoff(Role::Visitor);
        agents[host_id].add_strategy_payoff(Role::Host);
    }
}

fn main() {
    let seed = 0;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let max_time_step: usize = 100000000;
    let pop: usize = 20;

    let mut agents: Vec<Agent> = Vec::new();
    let mut network: Network = Network(vec![vec![0.5; pop]; pop]);
    let payoffs: PayoffMap = PayoffMap::new();

    for i in 0..pop {
        agents.push(Agent::new(AgentId(i as u32), rng.random()));
    }

    for _i in 1..=max_time_step {
        run_time_step(&mut rng, &mut agents, pop, &mut network, &payoffs);
    }
}
