use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub mods_dir: String,
    pub deps_dir: String,
    pub secret: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn load_config_from_env() -> Config {
    dotenvy::dotenv().ok();

    envy::from_env::<Config>()
        .expect("Failed to load config")
}

pub fn load_config() -> &'static Config {
    CONFIG.get_or_init(load_config_from_env)
}