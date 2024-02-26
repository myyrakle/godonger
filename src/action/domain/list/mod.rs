use crate::{
    command::domain::list::ConfigOptions,
    config::{self},
};

mod tui;

pub fn run(_: ConfigOptions) {
    if !config::exists_config() {
        eprintln!("Config not found. Please run `init` first.");
        std::process::exit(1);
    }

    let config_file = config::load_config();

    let domain_path = config_file.domain_dir;

    let domain_file_path_list = if !domain_path.exists() {
        vec![]
    } else {
        domain_path
            .read_dir()
            .expect("Failed to read domain directory")
            .map(|entry| entry.expect("Failed to read entry").path())
            .collect()
    };

    let dto_file_suffix = &config_file.dto_file_suffix;
    let model_file_suffix = &config_file.model_file_suffix;
    let args_file_suffix = &config_file.args_file_suffix;

    let domain_list: Vec<String> = domain_file_path_list
        .iter()
        .filter_map(|path| {
            let file_name = path.file_name()?.to_str()?;
            let name = file_name.split('.').next()?;
            let file_type = file_name.split('.').last()?;

            if file_type != "go" {
                return None;
            }

            if name.ends_with("_test") {
                return None;
            }

            if name.ends_with(dto_file_suffix) {
                return None;
            }

            if name.ends_with(model_file_suffix) {
                return None;
            }

            if name.ends_with(args_file_suffix) {
                return None;
            }

            for exclude in &config_file.exclude_file_suffix_list {
                if name.ends_with(exclude) {
                    return None;
                }
            }

            Some(name.into())
        })
        .collect();

    tui::run(domain_list).unwrap();
}
