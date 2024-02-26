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

    handler_struct_suffix: String,

    usecase_struct_suffix: String,
    usecase_interface_suffix: String,
    request_dto_struct_suffix: String,
    response_dto_struct_suffix: String,

    store_variant_list: Vec<String>,
    store_struct_suffix: String,
    store_interface_suffix: String,
    params_struct_suffix: String,

    helper_struct_suffix: String,
    helper_interface_suffix: String,
    args_struct_suffix: String,
    result_struct_suffix: String,

    framework: Framework,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            domain_dir: PathBuf::new().join("./domain"),
            internal_dir: PathBuf::new().join("./internal"),
            entrypoint_file: PathBuf::new().join("./cmd/main.go"),
            handler_struct_suffix: "Handler".to_string(),
            usecase_struct_suffix: "Usecase".to_string(),
            usecase_interface_suffix: "Usecase".to_string(),
            request_dto_struct_suffix: "Request".to_string(),
            response_dto_struct_suffix: "Response".to_string(),
            store_struct_suffix: "Store".to_string(),
            store_interface_suffix: "Store".to_string(),
            helper_struct_suffix: "Helper".to_string(),
            helper_interface_suffix: "Helper".to_string(),
            store_variant_list: vec![
                "Memory".to_string(),
                "Mongo".to_string(),
                "Postgresql".to_string(),
                "Http".to_string(),
            ],
            params_struct_suffix: "Params".to_string(),
            args_struct_suffix: "Args".to_string(),
            result_struct_suffix: "Result".to_string(),
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
