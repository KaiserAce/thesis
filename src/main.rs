mod utils;

use rand::prelude::{IndexedRandom, SliceRandom};
use rand::{Rng, rng};
use rayon::prelude::*;
use std::env;
use std::ops::{Index, IndexMut};
use utils::{
    AgentInteractionTracker, AgentParameters, InteractionTracker, PayoffScores, RootConfig,
    create_directories, delete_directories, get_config_files, read_config_file, run_track_vars,
};

enum Role {
    Host,
    Visitor,
}

#[derive(Clone, Copy)]
enum Strategy {
    None,
    Hawk,
    Dove,
    Fox,
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
    agent_param: AgentParameters,
    score: f64,
    total_payoff: f64,
    current_partner: AgentId,
    strategy: StratVector,
    current_strategy: Strategy,
    current_payoff: f64,
    morality_rate: f64,
    deviancy_rate: f64,
    deviant: bool,
}

struct StratVector {
    visit: Vec<f64>,
    host: Vec<f64>,
}

struct PayoffMap {
    hh: f32,
    hd: f32,
    dh: f32,
    dd: f32,
    fh: f32,
    hf: f32,
    fd: f32,
    df: f32,
    win: f32,
    lose: f32,
}

impl Agent {
    fn new(agent_id: AgentId, score: f64, agent_param: AgentParameters) -> Agent {
        Agent {
            agent_id,
            agent_param,
            score,
            total_payoff: 0.0,
            current_partner: AgentId(0),
            strategy: StratVector::new(),
            current_strategy: Strategy::None,
            current_payoff: 0.0,
            morality_rate: 1.0,
            deviancy_rate: 1.0,
            deviant: false,
        }
    }

    fn partner_pick(&mut self, temp_vec: &[usize], network: &mut Network) -> AgentId {
        let mut rng = rng();
        let mut friend_id: Option<usize> = None;
        let rand_tremble: f64 = rng.random();

        if rand_tremble < self.agent_param.net_tremble {
            for i in 0..temp_vec.len() {
                network.discount_weight(
                    self.agent_id,
                    AgentId(i as u32),
                    self.agent_param.net_discount,
                );
            }

            let partner_id: AgentId = AgentId(*temp_vec.choose(&mut rng).unwrap() as u32);
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

                network.discount_weight(
                    self.agent_id,
                    AgentId(i as u32),
                    self.agent_param.net_discount,
                );
            }
            let partner_id: AgentId =
                AgentId(friend_id.unwrap_or_else(|| *temp_vec.choose(&mut rng).unwrap()) as u32);
            self.current_partner = partner_id;
            partner_id
        }
    }

    fn choose_strategy(&mut self, role: Role) {
        let mut rng = rng();

        let rand_prob: f64 = rng.random();
        let deviancy_draw: f64 = rand_prob * (self.deviancy_rate + self.morality_rate);

        if deviancy_draw < self.morality_rate {
            self.deviant = false;
        } else {
            self.deviant = true;
        }

        // match (deviancy_draw > self.morality_rate, &role) {
        //     (true, Role::Visitor) => self.deviant = true,
        //     (false, _ ) => self.deviant = false,
        //     ( _ , Role::Host) => self.deviant = false,
        // }

        let tremble_draw: f64 = rng.random();

        if tremble_draw < self.agent_param.strat_tremble {
            let strategy = match (self.deviant, &role) {
                (true, Role::Visitor) => rng.random_range(0..3),
                (false, _) | (true, Role::Host) => rng.random_range(0..2),
            };

            self.current_strategy = match strategy {
                0 => Strategy::Hawk,
                1 => Strategy::Dove,
                2 => Strategy::Fox,
                _ => unreachable!(),
            };

            let strat_vec = self.strategy.get_strat(&role);
            strat_vec
                .iter_mut()
                .for_each(|x| *x *= 1.0 - self.agent_param.strat_discount);

        } else {
            let rand_prob: f64 = rng.random();

            let mut strat_key: Vec<u8> = match (self.deviant, &role) {
                (true, Role::Visitor) => (0..3).collect(),
                (false, _) | (true, Role::Host) => (0..2).collect(),
            };

            strat_key.shuffle(&mut rng);

            let strat_vec = self.strategy.get_strat(&role);

            let mut partial_sum: Vec<f64> = Vec::new();
            let mut acc: f64 = 0.0;

            for i in &strat_key {
                acc += strat_vec[*i as usize];
                partial_sum.push(acc);
            }

            let weight_strat_draw = rand_prob * partial_sum.last().unwrap_or(&0.0);

            for i in 0..partial_sum.len() {
                if weight_strat_draw <= partial_sum[i] {
                    let strategy = &strat_key[i];
                    self.current_strategy = match strategy {
                        0 => Strategy::Hawk,
                        1 => Strategy::Dove,
                        2 => Strategy::Fox,
                        _ => unreachable!(),
                    };
                    break;
                }
            }

            for i in &strat_key {
                strat_vec[*i as usize] *= 1.0 - self.agent_param.strat_discount;
            }
        }
    }

    fn add_network_payoff(&mut self, network: &mut Network) {
        network[self.agent_id][self.current_partner.0 as usize] +=
            self.current_payoff * self.agent_param.net_learning_speed;
    }

    fn add_strategy_payoff(&mut self, role: Role) {
        let index = match self.current_strategy {
            Strategy::Hawk => 0,
            Strategy::Dove => 1,
            Strategy::Fox => 2,
            _ => unreachable!(),
        };

        match role {
            Role::Visitor => {
                self.strategy.visit[index] +=
                    self.current_payoff * self.agent_param.strat_learning_speed
            }
            Role::Host => {
                self.strategy.host[index] +=
                    self.current_payoff * self.agent_param.strat_learning_speed
            }
        }

        self.total_payoff += self.current_payoff
    }

    fn update_score(&mut self) {
        self.score = self.total_payoff;
    }

    fn discount_morality(&mut self) {
        self.morality_rate *= 1.0 - self.agent_param.strat_discount;
        self.deviancy_rate *= 1.0 - self.agent_param.strat_discount;
    }

    fn add_morality_payoff(&mut self) {
        match self.deviant {
            true => self.deviancy_rate += self.current_payoff * self.agent_param.strat_learning_speed,
            false => self.morality_rate += self.current_payoff * self.agent_param.strat_learning_speed,
        }
    }
}

impl StratVector {
    fn new() -> StratVector {
        StratVector {
            visit: vec![1.0, 1.0, 1.0],
            host: vec![1.0, 1.0],
        }
    }

    fn get_strat(&mut self, role: &Role) -> &mut Vec<f64> {
        match role {
            Role::Host => &mut self.host,
            Role::Visitor => &mut self.visit,
        }
    }
}

impl Network {
    fn new(pop: usize) -> Network {
        let mut network_weights = vec![vec![1.0 / (pop - 1) as f64; pop]; pop];

        for i in 0..pop {
            network_weights[i][i] = 0.0;
        }

        Network(network_weights)
    }

    fn discount_weight(&mut self, agent_id: AgentId, partner_id: AgentId, net_discount: f64) {
        self.0[agent_id.0 as usize][partner_id.0 as usize] *= 1.0 - net_discount;
    }

    fn normalize_network_weights(&mut self, agent_id: AgentId) {
        let sum: f64 = self[agent_id].iter().sum();

        for weight in self[agent_id].iter_mut() {
            *weight /= sum;
        }
    }
}

impl PayoffMap {
    fn new(payoff: PayoffScores) -> PayoffMap {
        PayoffMap {
            hh: 0.0,
            hd: payoff.hd,
            dh: payoff.dh,
            dd: payoff.dd,
            fh: payoff.fh,
            hf: payoff.hf,
            fd: payoff.fd,
            df: payoff.df,
            win: payoff.hh_f,
            lose: payoff.hh_f / 3.0,
        }
    }
}

fn game(
    visitor: AgentId,
    host: AgentId,
    agents: &mut Vec<Agent>,
    payoffs: &PayoffMap,
    interaction_tracker: &mut InteractionTracker,
    agent_interaction_tracker: &mut Vec<AgentInteractionTracker>,
) {
    let visitor_score: f64 = agents[visitor].score;
    let host_score: f64 = agents[host].score;

    let visitor_strategy: Strategy = agents[visitor].current_strategy;
    let host_strategy: Strategy = agents[host].current_strategy;

    agent_interaction_tracker[visitor].fox_this_round = false;

    match (visitor_strategy, host_strategy) {
        (Strategy::Hawk, Strategy::Hawk) => {
            interaction_tracker.hawk_hawk += 1;
            agent_interaction_tracker[visitor].hawk_hawk += 1;
            agent_interaction_tracker[host].hawk_hawk += 1;
            if visitor_score > host_score {
                agents[visitor].current_payoff = payoffs.win as f64;
                agents[host].current_payoff = payoffs.lose as f64;
            } else {
                agents[visitor].current_payoff = payoffs.lose as f64;
                agents[host].current_payoff = payoffs.win as f64;
            }
        }
        (Strategy::Hawk, Strategy::Dove) => {
            agents[visitor].current_payoff = payoffs.hd as f64;
            agents[host].current_payoff = payoffs.dh as f64;
            interaction_tracker.hawk_dove += 1;
            agent_interaction_tracker[visitor].hawk_dove += 1;
            agent_interaction_tracker[host].hawk_dove += 1;
        }
        (Strategy::Dove, Strategy::Hawk) => {
            agents[visitor].current_payoff = payoffs.dh as f64;
            agents[host].current_payoff = payoffs.hd as f64;
            interaction_tracker.dove_hawk += 1;
            agent_interaction_tracker[visitor].dove_hawk += 1;
            agent_interaction_tracker[host].dove_hawk += 1;
        }
        (Strategy::Dove, Strategy::Dove) => {
            agents[visitor].current_payoff = payoffs.dd as f64;
            agents[host].current_payoff = payoffs.dd as f64;
            interaction_tracker.dove_dove += 1;
            agent_interaction_tracker[visitor].dove_dove += 1;
            agent_interaction_tracker[host].dove_dove += 1;
        }
        (Strategy::Fox, Strategy::Hawk) => {
            agents[visitor].current_payoff = payoffs.fh as f64;
            agents[host].current_payoff = payoffs.hf as f64;
            interaction_tracker.fox_hawk += 1;
            agent_interaction_tracker[visitor].fox_hawk += 1;
            agent_interaction_tracker[visitor].fox_this_round = true;
            agent_interaction_tracker[host].fox_hawk += 1;
        }
        (Strategy::Fox, Strategy::Dove) => {
            agents[visitor].current_payoff = payoffs.fd as f64;
            agents[host].current_payoff = payoffs.df as f64;
            interaction_tracker.fox_dove += 1;
            agent_interaction_tracker[visitor].fox_dove += 1;
            agent_interaction_tracker[visitor].fox_this_round = true;
            agent_interaction_tracker[host].fox_dove += 1;
        }
        _ => unreachable!(),
    }
}

fn run_time_step(
    i: u64,
    agents: &mut Vec<Agent>,
    pop: usize,
    network: &mut Network,
    payoffs: &PayoffMap,
    dynamic_rank: bool,
    seed: u64,
    output_directory: &str,
    agent_interaction_tracker: &mut Vec<AgentInteractionTracker>,
    config: &RootConfig,
) {
    let mut agent_seq: Vec<usize> = (0..pop).collect();
    let mut rng = rng();
    agent_seq.shuffle(&mut rng);

    let mut interaction_tracker: InteractionTracker = InteractionTracker::new();

    for &id in &agent_seq {
        let temp_vec: Vec<usize> = agent_seq.iter().filter(|&&x| x != id).cloned().collect();

        let host_id: AgentId = agents[id].partner_pick(&temp_vec, network);

        {
            let visitor = &mut agents[id];
            visitor.choose_strategy(Role::Visitor);
            visitor.current_partner = host_id;
        }

        {
            let host = &mut agents[host_id.0 as usize];
            host.choose_strategy(Role::Host);
        }

        game(
            AgentId(id as u32),
            host_id,
            agents,
            payoffs,
            &mut interaction_tracker,
            agent_interaction_tracker,
        );

        agents[id].add_network_payoff(network);

        agents[id].add_strategy_payoff(Role::Visitor);
        agents[host_id].add_strategy_payoff(Role::Host);

        agents[id].discount_morality();
        agents[id].add_morality_payoff();

        agents[host_id].discount_morality();
        agents[host_id].add_morality_payoff();

        network.normalize_network_weights(AgentId(id as u32));
    }

    if dynamic_rank {
        if i % 1000 == 0 {
            for j in 0..pop {
                agents[j].update_score();
            }
        }
    }

    run_track_vars(
        i,
        pop,
        agents,
        network,
        output_directory,
        seed,
        &interaction_tracker,
        agent_interaction_tracker,
        config,
    );
}

fn run_config_file(config: &RootConfig, out_path: &str) {
    let seeds = config.simulation.seeds;
    let mut rng = rng();
    let max_time_step: u64 = config.simulation.max_time_step;
    let pop: u32 = config.simulation.population;
    let dynamic_rank: bool = config.simulation.dynamic_rank;
    let output_directory: String = config.simulation.output_directory.clone();

    let payoffs = PayoffMap::new(config.payoffs);

    let work_direc = format!("{}/{}", out_path, output_directory);

    let _ = delete_directories(&work_direc);
    create_directories(&work_direc);

    for seed in 0..seeds {
        let mut agents: Vec<Agent> = Vec::new();
        let mut network = Network::new(pop as usize);
        let mut agent_interaction_tracker: Vec<AgentInteractionTracker> =
            vec![AgentInteractionTracker::new(); pop as usize];

        for i in 0..pop {
            agents.push(Agent::new(
                AgentId(i as u32),
                rng.random(),
                config.agent_parameters,
            ));
        }

        run_track_vars(
            0 as u64,
            pop as usize,
            &agents,
            &network,
            &work_direc,
            seed,
            &InteractionTracker::default(pop as usize),
            &agent_interaction_tracker,
            config,
        );

        for i in 1..=max_time_step {
            run_time_step(
                i,
                &mut agents,
                pop as usize,
                &mut network,
                &payoffs,
                dynamic_rank,
                seed,
                &work_direc,
                &mut agent_interaction_tracker,
                config,
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Wrong arguments entered!")
    }

    let path = get_config_files(&args[1]);

    path.par_iter().for_each(|file| {
        print!("{}", file);
        let config: RootConfig = read_config_file(&format!("/{}/{}", &args[1], &file));
        println!("{}", config.description);
        run_config_file(&config, &args[1]);
    });
}
