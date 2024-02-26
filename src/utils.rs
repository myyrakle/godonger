use crate::config::{self, RootConfig};

pub fn get_config_file_or_warn() -> RootConfig {
    if !config::exists_config() {
        eprintln!("Config not found. Please run `init` first.");
        std::process::exit(1);
    }

    config::load_config()
}
