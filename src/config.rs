use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
}

pub fn get_config() -> Config{
    let text: String = std::fs::read_to_string("config.yaml").expect("couldn't read config.yaml");
    let config: Config = serde_yaml::from_str(&text).expect("invalid config.yaml");
    config
}