use crate::config::RootConfig;
use convert_case::{Case, Casing};

pub fn generate_usecase_inteface_name(domain: String, config_file: &RootConfig) -> String {
    let usecase_interface_name =
        domain.as_str().to_case(Case::Pascal) + &config_file.usecase_interface_suffix;

    usecase_interface_name
}
