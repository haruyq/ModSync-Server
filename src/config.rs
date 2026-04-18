use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub mods_dir: String,
    pub deps_dir: String,
    pub secret: String,
}

pub fn load_config() -> Config {
    dotenvy::dotenv().ok();

    envy::from_env::<Config>()
        .expect("Failed to load config")
}