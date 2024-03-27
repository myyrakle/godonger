use convert_case::{Case, Casing};
use std::fs;

use crate::utils::get_config_file_or_warn;

use super::boilerplates::{
    find_interface_and_append_method, generate_request_dto, generate_response_dto,
    generate_usecase_filepath, generate_usecase_interface_method, generate_usecase_interface_type,
    generate_usecase_method,
};

pub fn add_usecase_interface_to_domain_file_if_not_exists(domain: String) {
    let config_file = get_config_file_or_warn();

    let domain_file_path = config_file.domain_dir.join(domain.clone() + ".go");

    let usecase_interface_name = generate_usecase_interface_type(domain.clone(), &config_file);

    let mut domain_file_content = fs::read_to_string(&domain_file_path).unwrap();

    if !domain_file_content.contains(&usecase_interface_name) {
        domain_file_content.push_str("\n\n");

        domain_file_content.push_str(format!(r#"// {usecase_interface_name}\n"#).as_str());
        domain_file_content
            .push_str(format!(r#"type {usecase_interface_name} interface {{}}"#).as_str());

        fs::write(domain_file_path, domain_file_content).unwrap();
    }
}

pub fn add_method_to_usecase_interface_of_domain_file(domain: String, method_name: String) {
    let config_file = get_config_file_or_warn();

    let domain_file_path = config_file.domain_dir.join(domain.clone() + ".go");

    let usecase_interface_type = generate_usecase_interface_type(domain.clone(), &config_file);

    let domain_file_content = fs::read_to_string(&domain_file_path).unwrap();

    let method_code = generate_usecase_interface_method(method_name, true, &config_file);

    if let Some(_) = domain_file_content.find(&usecase_interface_type) {
        let replaced_content = find_interface_and_append_method(
            &domain_file_content,
            &usecase_interface_type,
            &method_code,
        );

        fs::write(domain_file_path, replaced_content).unwrap();
    } else {
        panic!("usecase interface {usecase_interface_type} not found in domain file")
    }
}

pub fn add_method_to_usecase_of_usecase_file(domain: &str, method_name: &str) {
    let config_file = get_config_file_or_warn();

    let usecase_file_path = generate_usecase_filepath(domain, &config_file);

    let method_code = generate_usecase_method(domain, method_name, true, &config_file);

    let mut usecase_file_content = fs::read_to_string(&usecase_file_path).unwrap();

    usecase_file_content.push_str("\n\n");
    usecase_file_content.push_str(&method_code);

    fs::write(usecase_file_path, usecase_file_content).unwrap();
}

pub fn add_dto_types_to_domain_file(domain: String, method_name: String, has_response: bool) {
    let config_file = get_config_file_or_warn();

    let domain_dto_filename =
        domain.as_str().to_case(Case::Snake) + &config_file.dto_file_suffix + ".go";

    let domain_dto_filepath = config_file.domain_dir.join(&domain_dto_filename);

    let mut domain_dto_file_content = fs::read_to_string(&domain_dto_filepath).unwrap();

    let request_dto_type = generate_request_dto(&method_name, &config_file);

    domain_dto_file_content.push_str(&request_dto_type);
    domain_dto_file_content.push_str("\n\n");

    if has_response {
        let response_dto_type = generate_response_dto(&method_name, &config_file);

        domain_dto_file_content.push_str(&response_dto_type);
        domain_dto_file_content.push_str("\n\n");
    }

    fs::write(domain_dto_filepath, domain_dto_file_content).unwrap();
}

pub fn add_method_to_handler_file(
    domain: &str,
    _api_path: &str,
    method_name: &str,
    has_response: bool,
) {
    let config_file = get_config_file_or_warn();

    let handler_file_path = config_file.route_http_dir.join(domain.to_owned() + ".go");

    let method_code = generate_usecase_method(domain, method_name, has_response, &config_file);

    let mut handler_file_content = fs::read_to_string(&handler_file_path).unwrap();

    handler_file_content.push_str("\n\n");
    handler_file_content.push_str(&method_code);

    fs::write(handler_file_path, handler_file_content).unwrap();
}
