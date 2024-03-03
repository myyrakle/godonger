use std::fs;

use crate::utils::get_config_file_or_warn;

use super::boilerplates::generate_usecase_inteface_name;

pub fn add_usecase_interface_to_domain_file_if_not_exists(domain: String) {
    let config_file = get_config_file_or_warn();

    let domain_file_path = config_file.domain_dir.join(domain.clone() + ".go");

    let usecase_interface_name = generate_usecase_inteface_name(domain.clone(), &config_file);

    let mut domain_file_content = fs::read_to_string(&domain_file_path).unwrap();

    if !domain_file_content.contains(&usecase_interface_name) {
        domain_file_content.push_str("\n\n");

        domain_file_content.push_str(format!(r#"// {usecase_interface_name}\n"#).as_str());
        domain_file_content
            .push_str(format!(r#"type {usecase_interface_name} interface {{}}"#).as_str());

        fs::write(domain_file_path, domain_file_content).unwrap();
    }
}

pub fn add_method_to_usecase_interface_of_domain_file(domain: String, method_name: String) {}
