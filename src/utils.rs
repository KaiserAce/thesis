use core::panic;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimulationParameters {
    pub seed: u64,
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

#[derive(Debug, Deserialize)]
pub struct PayoffScores {
    pub hd: f32,
    pub dh: f32,
    pub dd: f32,
    pub hh_f: f32,
}

#[derive(Debug, Deserialize)]
pub struct RootConfig {
    _description: String,
    pub simulation: SimulationParameters,
    pub agent: AgentParameters,
    pub payoffs: PayoffScores,
}

impl RootConfig {
    fn new(source_file: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&format!("./Input/{}", source_file)))
            .build()?;
        s.try_deserialize()
    }
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
