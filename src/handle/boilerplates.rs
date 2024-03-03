use crate::config::RootConfig;
use convert_case::{Case, Casing};

pub fn generate_usecase_interface_name(domain: String, config_file: &RootConfig) -> String {
    let usecase_interface_name =
        domain.as_str().to_case(Case::Pascal) + &config_file.usecase_interface_suffix;

    usecase_interface_name
}

pub fn generate_request_dto_name(method_name: String, config_file: &RootConfig) -> String {
    let request_dto_name =
        method_name.as_str().to_case(Case::Pascal) + &config_file.request_dto_struct_suffix;

    request_dto_name
}

pub fn generate_response_dto_name(method_name: String, config_file: &RootConfig) -> String {
    let response_dto_name =
        method_name.as_str().to_case(Case::Pascal) + &config_file.response_dto_struct_suffix;

    response_dto_name
}

pub fn generate_usecase_interface_method(method_name: String, has_response: bool) -> String {
    let method_name = method_name.as_str().to_case(Case::Pascal);

    let method_signature = if has_response {
        format!(r#"{}() (string, error)"#, method_name)
    } else {
        format!(r#"{}() error"#, method_name)
    };

    method_signature
}
