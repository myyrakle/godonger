use crate::config::{self, RootConfig};

pub fn get_config_file_or_warn() -> RootConfig {
    if !config::exists_config() {
        eprintln!("Config not found. Please run `init` first.");
        std::process::exit(1);
    }

    config::load_config()
}

pub fn get_scroll_point(selected_index: usize) -> (u16, u16) {
    let vertical = if selected_index > 5 {
        selected_index as u16 - 5
    } else {
        0
    };

    (vertical, 0)
}
