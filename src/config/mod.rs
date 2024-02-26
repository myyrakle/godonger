use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Framework {
    #[serde(rename = "echo")]
    Echo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RootConfig {
    domain_dir: PathBuf,
    internal_dir: PathBuf,
    entrypoint_file: PathBuf,
    framework: Framework,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            domain_dir: PathBuf::new().join("./domain"),
            internal_dir: PathBuf::new().join("./internal"),
            entrypoint_file: PathBuf::new().join("./cmd/main.go"),
            framework: Framework::Echo,
        }
    }
}

impl RootConfig {
    pub fn write(&self) {
        let config = serde_yaml::to_string(&self).expect("Failed to serialize config");
        std::fs::write(CONFIG_DEFAULT_PATH, config).expect("Failed to write config file");
    }
}

const CONFIG_DEFAULT_PATH: &str = "./.godonger.yaml";

pub fn load_config() -> RootConfig {
    let config = std::fs::read_to_string(CONFIG_DEFAULT_PATH).expect("Failed to read config file");
    serde_yaml::from_str(&config).expect("Failed to parse config file")
}

pub fn exists_config() -> bool {
    PathBuf::from_str(CONFIG_DEFAULT_PATH).unwrap().exists()
}
