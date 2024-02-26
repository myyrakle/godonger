use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Framework {
    #[serde(rename = "echo")]
    Echo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RootConfig {
    pub domain_dir: PathBuf,
    pub internal_dir: PathBuf,

    pub entrypoint_file: PathBuf,

    pub handler_struct_suffix: String,
    pub handler_file_suffix: String,
    pub handler_dir: PathBuf,

    pub usecase_struct_suffix: String,
    pub usecase_interface_suffix: String,
    pub usecase_file_suffix: String,
    pub usecase_dir: PathBuf,

    pub request_dto_struct_suffix: String,
    pub response_dto_struct_suffix: String,

    pub store_variant_list: Vec<String>,
    pub store_struct_suffix: String,
    pub store_interface_suffix: String,
    pub store_file_suffix: String,
    pub store_dir: PathBuf,

    pub params_struct_suffix: String,

    pub helper_struct_suffix: String,
    pub helper_interface_suffix: String,
    pub helper_file_suffix: String,
    pub helper_dir: PathBuf,

    pub args_struct_suffix: String,
    pub result_struct_suffix: String,

    pub dto_file_suffix: String,
    pub model_file_suffix: String,
    pub args_file_suffix: String,
    pub exclude_file_suffix_list: Vec<String>,

    pub framework: Framework,
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
            dto_file_suffix: "_dto".to_string(),
            model_file_suffix: "_model".to_string(),
            args_file_suffix: "_args".to_string(),
            handler_file_suffix: "_handler".to_string(),
            usecase_file_suffix: "_usecase".to_string(),
            store_file_suffix: "_store".to_string(),
            helper_file_suffix: "_helper".to_string(),
            exclude_file_suffix_list: vec!["_field".to_string()],
            usecase_dir: PathBuf::new().join("usecase"),
            handler_dir: PathBuf::new().join("route"),
            store_dir: PathBuf::new().join("store"),
            helper_dir: PathBuf::new().join("helper"),
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
