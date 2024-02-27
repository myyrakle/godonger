use convert_case::{Case, Casing};
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

pub fn add_usecase_interface_to_domain_file_if_not_exists(domain: String) {
    let config_file = get_config_file_or_warn();

    let domain_file_path = config_file.domain_dir.join(domain.clone() + ".go");

    let usecase_interface_name =
        domain.as_str().to_case(Case::Pascal) + &config_file.usecase_interface_suffix;

    let mut domain_file_content = fs::read_to_string(&domain_file_path).unwrap();

    if !domain_file_content.contains(&usecase_interface_name) {
        domain_file_content.push_str("\n\n");

        domain_file_content.push_str(format!(r#"// {usecase_interface_name}\n"#).as_str());
        domain_file_content
            .push_str(format!(r#"type {usecase_interface_name} interface {{}}"#).as_str());

        fs::write(domain_file_path, domain_file_content).unwrap();
    }
}

pub fn create_handler_file_if_not_exists(domain: String) {
    let config_file = get_config_file_or_warn();

    let handler_filename = domain.as_str().to_case(Case::Snake) + &config_file.route_file_suffix;

    let handler_file_path = config_file
        .internal_dir
        .join(domain.clone())
        .join(&config_file.route_dir)
        .join(&config_file.route_http_dir)
        .join(&handler_filename);

    if !handler_file_path.exists() {
        let package_name = config_file.route_http_dir.to_str().unwrap().to_string();

        let handler_type = domain.as_str().to_case(Case::Pascal) + &config_file.route_struct_suffix;

        let usecase_interface_type =
            domain.as_str().to_case(Case::Pascal) + &config_file.usecase_interface_suffix;

        let usecase_field_name = usecase_interface_type.as_str().to_case(Case::Camel);

        let mut code = String::new();

        code.push_str(format!("package {package_name}\n\n").as_str());

        code.push_str(format!("type {handler_type} struct {{\n").as_str());
        code.push_str(format!("    {usecase_field_name} {usecase_interface_type}\n").as_str());
        code.push_str("}\n\n");

        code.push_str(
            format!(
                "func New(echo *echo.Echo, {usecase_field_name} domain.{usecase_interface_type}) *{handler_type} {{\n"
            )
            .as_str(),
        );
        code.push_str(format!("    handler := &{handler_type}{{\n",).as_str());
        code.push_str(format!("        {usecase_field_name}: {usecase_field_name},\n",).as_str());
        code.push_str("    }\n\n");

        code.push_str("    return handler\n");
        code.push_str("}\n");
    }
}
