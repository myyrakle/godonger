use std::str::pattern::Pattern;

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

pub fn split_to_pair<'a, P>(s: String, pattern: &str) -> Option<(String, String)> {
    let mut split = s.split(pattern);

    let first = split.next()?.to_owned();

    let mut second = split.next()?.to_owned();

    while let Some(s) = split.next() {
        second = second.to_string() + s;
    }

    Some((first, second))
}
