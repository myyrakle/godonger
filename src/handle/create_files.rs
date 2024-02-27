use std::fs;

use crate::utils::get_config_file_or_warn;

pub fn create_domain_file_if_not_exists(domain: String) {
    let config_file = get_config_file_or_warn();

    let domain_file_path = config_file.domain_dir.join(domain + ".go");

    if !domain_file_path.exists() {
        // write empty domain file
        let code = "package domain";

        fs::write(domain_file_path, code).unwrap();
    }
}
