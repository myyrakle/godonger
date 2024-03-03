use std::fs;

use crate::utils::get_config_file_or_warn;

use super::boilerplates::{
    find_interface_and_append_method, generate_usecase_interface_method,
    generate_usecase_interface_type,
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
