use core::panic;
use std::io;

use config::{Config, ConfigError, File};
use csv::WriterBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::fs::{create_dir_all, remove_dir_all};
use std::ops::{Index, IndexMut};
use walkdir::WalkDir;

use crate::{Agent, AgentId, Network};

impl Index<AgentId> for Vec<AgentInteractionTracker> {
    type Output = AgentInteractionTracker;

    fn index(&self, index: AgentId) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl IndexMut<AgentId> for Vec<AgentInteractionTracker> {
    fn index_mut(&mut self, index: AgentId) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimulationParameters {
    pub seeds: u64,
    pub max_time_step: u64,
    pub population: u32,
    pub dynamic_rank: bool,
    pub output_directory: String,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct AgentParameters {
    pub strat_learning_speed: f64,
    pub net_learning_speed: f64,
    pub strat_discount: f64,
    pub net_discount: f64,
    pub strat_tremble: f64,
    pub net_tremble: f64,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PayoffScores {
    pub hd: f32,
    pub dh: f32,
    pub dd: f32,
    pub hh_f: f32,
}

#[derive(Debug, Deserialize)]
pub struct CSVFiles {
    pub weights: bool,
    pub scores: bool,
    pub totalinteractions: bool,
    pub evostats: bool,
    pub strategyvisit: bool,
    pub strategyhost: bool,
    pub netstd: bool,
    pub outscore: bool,
    pub totalpayoff: bool,
}

#[derive(Debug, Deserialize)]
pub struct RootConfig {
    pub description: String,
    pub simulation: SimulationParameters,
    pub agent_parameters: AgentParameters,
    pub payoffs: PayoffScores,
    pub csv: CSVFiles,
}

pub struct InteractionTracker {
    pub hawk_hawk: u64,
    pub hawk_dove: u64,
    pub dove_hawk: u64,
    pub dove_dove: u64,
}

#[derive(Clone, Copy)]
pub struct AgentInteractionTracker {
    pub hawk_hawk: u64,
    pub hawk_dove: u64,
    pub dove_hawk: u64,
    pub dove_dove: u64,
}

impl RootConfig {
    fn new(source_file: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&format!("./Input/{}", source_file)))
            .build()?;
        s.try_deserialize()
    }
}

impl InteractionTracker {
    pub fn new() -> InteractionTracker {
        InteractionTracker {
            hawk_hawk: 0,
            hawk_dove: 0,
            dove_hawk: 0,
            dove_dove: 0,
        }
    }

    pub fn default(pop: usize) -> InteractionTracker {
        InteractionTracker {
            hawk_hawk: pop as u64 / 4,
            hawk_dove: pop as u64 / 4,
            dove_hawk: pop as u64 / 4,
            dove_dove: pop as u64 / 4,
        }
    }
}

impl AgentInteractionTracker {
    pub fn new() -> AgentInteractionTracker {
        AgentInteractionTracker {
            hawk_hawk: 0,
            hawk_dove: 0,
            dove_hawk: 0,
            dove_dove: 0,
        }
    }
}

pub fn get_config_files(path: &str) -> Vec<String> {
    let full_path = format!("Input/{}", path);

    WalkDir::new(full_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

pub fn read_config_file(source_file: &String) -> RootConfig {
    match RootConfig::new(source_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading simulation parameters: {}", e);
            panic!("Failed to load simulation parameters");
        }
    }
}

pub fn delete_directories(output_directory: &str) -> io::Result<()> {
    let path = format!("./Output/{}", output_directory);
    match remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn create_directories(output_directory: &str) {
    let path = format!("./Output/{}", output_directory);
    match create_dir_all(path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error making directory: {}", e);
        }
    }
}

pub fn generate_weights_csv(
    i: u64,
    network: &Network,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/Weights_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let linear_vec: Vec<f32> = network
        .0
        .iter()
        .flat_map(|row| row.iter().cloned())
        .map(|x| x as f32)
        .collect();

    let mut string_vec: Vec<String> = linear_vec.iter().map(|x| x.to_string()).collect();

    string_vec.insert(0, i.to_string());

    writer.write_record(&string_vec)?;

    Ok(())
}

pub fn generate_evostats_csv(
    i: u64,
    pop: usize,
    interaction_tracker: &InteractionTracker,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/EvoStats_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut strat_stats = vec![i.to_string()];

    let hawk_hawk = (interaction_tracker.hawk_hawk as f64 / pop as f64).to_string();
    strat_stats.push(hawk_hawk);
    let hawk_dove = (interaction_tracker.hawk_dove as f64 / pop as f64).to_string();
    strat_stats.push(hawk_dove);
    let dove_hawk = (interaction_tracker.dove_hawk as f64 / pop as f64).to_string();
    strat_stats.push(dove_hawk);
    let dove_dove = (interaction_tracker.dove_dove as f64 / pop as f64).to_string();
    strat_stats.push(dove_dove);

    writer.write_record(&strat_stats)?;

    Ok(())
}

pub fn generate_strategyvisit_csv(
    agents: &Vec<Agent>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/StrategyVisit_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut visit_vector: Vec<f64> = Vec::new();

    for i in 0..agents.len() {
        let sum = agents[i].strategy.visit[0] + agents[i].strategy.visit[1];
        visit_vector.push(agents[i].strategy.visit[0] / sum);
        visit_vector.push(agents[i].strategy.visit[1] / sum);
    }

    let string_vec: Vec<String> = visit_vector.iter().map(|x| *x as f32).map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_strategyhost_csv(
    agents: &Vec<Agent>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/StrategyHost_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut host_vector: Vec<f64> = Vec::new();

    for i in 0..agents.len() {
        let sum = agents[i].strategy.host[0] + agents[i].strategy.host[1];
        host_vector.push(agents[i].strategy.host[0] / sum);
        host_vector.push(agents[i].strategy.host[1] / sum);
    }

    let string_vec: Vec<String> = host_vector.iter().map(|x| *x as f32).map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_scores_csv(
    agents: &Vec<Agent>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/Scores_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut scores_vector: Vec<f64> = Vec::new();

    for i in 0..agents.len() {
        scores_vector.push(agents[i].score);
    }

    let string_vec: Vec<String> = scores_vector.iter().map(|x| *x as f32).map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_netstd_csv(
    pop: usize,
    network: &Network,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/NetSTD_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut column_sums: Vec<f64> = vec![0.0; pop];

    for i in 0..pop {
        for j in 0..pop {
            column_sums[i] += network.0[j][i];
        }
    }

    let string_vec: Vec<String> = column_sums.iter().map(|x| *x as f32).map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_outscore_csv(
    pop: usize,
    agents: &Vec<Agent>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/OutScore_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);
    let mut scores: Vec<f64> = Vec::new();
    for i in 0..pop {
        scores.push(agents[i].score);
    }

    let mut indexed_data: Vec<(usize, &f64)> = scores.iter().enumerate().collect();
    indexed_data.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut ranks: HashMap<usize, usize> = HashMap::new();
    let mut rank = 1;

    for i in 0..indexed_data.len() {
        let (index, _) = indexed_data[i];
        if i > 0 && indexed_data[i].1 != indexed_data[i - 1].1 {
            rank = i + 1;
        }
        ranks.insert(index, rank);
    }

    let mut result = vec![0; scores.len()];

    for (original_index, &_value) in scores.iter().enumerate() {
        if let Some(&r) = ranks.get(&original_index) {
            result[original_index] = r;
        }
    }

    let string_vec: Vec<String> = result.iter().map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_totalpayoff_csv(
    pop: usize,
    agents: &Vec<Agent>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!("./Output/{}/TotalPayoff_{}.csv", output_directory, seed);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut payoff_vector: Vec<f64> = Vec::new();

    for i in 0..pop {
        payoff_vector.push(agents[i].total_payoff);
    }

    let string_vec: Vec<String> = payoff_vector.iter().map(|x| *x as f32).map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn generate_totalinteractions_csv(
    pop: usize,
    agent_interaction_tracker: &Vec<AgentInteractionTracker>,
    output_directory: &str,
    seed: u64,
) -> Result<(), Box<dyn Error>> {
    let filepath = format!(
        "./Output/{}/TotalInteractions_{}.csv",
        output_directory, seed
    );

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filepath)?;

    let mut writer = WriterBuilder::new().from_writer(file);

    let mut interaction_vector: Vec<u64> = Vec::new();

    for i in 0..pop {
        let hawk_hawk = agent_interaction_tracker[i].hawk_hawk;
        interaction_vector.push(hawk_hawk);
        let hawk_dove = agent_interaction_tracker[i].hawk_dove;
        interaction_vector.push(hawk_dove);
        let dove_hawk = agent_interaction_tracker[i].dove_hawk;
        interaction_vector.push(dove_hawk);
        let dove_dove = agent_interaction_tracker[i].dove_dove;
        interaction_vector.push(dove_dove);
    }

    let string_vec: Vec<String> = interaction_vector.iter().map(|x| x.to_string()).collect();

    writer.write_record(string_vec)?;

    Ok(())
}

pub fn run_track_vars(
    i: u64,
    pop: usize,
    agents: &Vec<Agent>,
    network: &Network,
    output_directory: &str,
    seed: u64,
    interaction_tracker: &InteractionTracker,
    agent_interaction_tracker: &Vec<AgentInteractionTracker>,
    config: &RootConfig,
) {
    let rounds = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 400, 500, 600,
        700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000,
        40000, 50000, 60000, 70000, 80000, 90000, 100000,
    ];

    if rounds.contains(&i) {
        if config.csv.weights {
            let _ = generate_weights_csv(i, network, output_directory, seed);
        }

        if config.csv.scores {
            let _ = generate_scores_csv(agents, output_directory, seed);
        }

        if config.csv.totalinteractions {
            let _ =
                generate_totalinteractions_csv(pop, agent_interaction_tracker, output_directory, seed);
        }
    }
    
    if config.csv.evostats {
        let _ = generate_evostats_csv(i, pop, interaction_tracker, output_directory, seed);
    }

    if config.csv.strategyvisit {
        let _ = generate_strategyvisit_csv(agents, output_directory, seed);
    }

    if config.csv.strategyhost {
        let _ = generate_strategyhost_csv(agents, output_directory, seed);
    }

    if config.csv.netstd {
        let _ = generate_netstd_csv(pop, network, output_directory, seed);
    }

    if config.csv.outscore {
        let _ = generate_outscore_csv(pop, agents, output_directory, seed);
    }

    if config.csv.totalpayoff {
        let _ = generate_totalpayoff_csv(pop, agents, output_directory, seed);
    }
}
