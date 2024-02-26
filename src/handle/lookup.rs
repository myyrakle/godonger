use std::path::PathBuf;

use crate::utils::get_config_file_or_warn;

pub fn lookup_domain(domain: String) -> Vec<PathBuf> {
    let config_file = get_config_file_or_warn();

    let domain_path = config_file.domain_dir;

    let main_file = domain.clone() + ".go";
    let args_file = domain.clone() + &config_file.args_file_suffix + ".go";
    let dto_file = domain.clone() + &config_file.dto_file_suffix + ".go";
    let model_file = domain + &config_file.model_file_suffix + ".go";

    let domain_file_path_list = if !domain_path.exists() {
        vec![]
    } else {
        domain_path
            .read_dir()
            .expect("Failed to read domain directory")
            .map(|entry| entry.expect("Failed to read entry").path())
            .filter(|path| {
                path.ends_with(&main_file)
                    || path.ends_with(&args_file)
                    || path.ends_with(&dto_file)
                    || path.ends_with(&model_file)
            })
            .collect()
    };

    domain_file_path_list
}

pub fn lookup_internal(domain: String) {
    let config_file = get_config_file_or_warn();
}
