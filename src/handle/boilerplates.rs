use crate::config::RootConfig;
use convert_case::{Case, Casing};

pub fn generate_usecase_struct_name(domain: &str, config_file: &RootConfig) -> String {
    let usecase_struct_name = domain.to_case(Case::Camel) + &config_file.usecase_struct_suffix;

    usecase_struct_name
}

pub fn generate_usecase_interface_type(domain: String, config_file: &RootConfig) -> String {
    let usecase_interface_name =
        domain.as_str().to_case(Case::Pascal) + &config_file.usecase_interface_suffix;

    usecase_interface_name
}

pub fn generate_request_dto_type(method_name: String, config_file: &RootConfig) -> String {
    let request_dto_name =
        method_name.as_str().to_case(Case::Pascal) + &config_file.request_dto_struct_suffix;

    request_dto_name
}

pub fn generate_response_dto_type(method_name: String, config_file: &RootConfig) -> String {
    let response_dto_name =
        method_name.as_str().to_case(Case::Pascal) + &config_file.response_dto_struct_suffix;

    response_dto_name
}

pub fn generate_usecase_interface_method(
    method_name: String,
    has_response: bool,
    config_file: &RootConfig,
) -> String {
    let method_name = method_name.as_str().to_case(Case::Pascal);

    let request_dto_type = generate_request_dto_type(method_name.clone(), config_file);

    let method_signature = if has_response {
        let response_dto_type = generate_response_dto_type(method_name.clone(), config_file);

        format!(
            r#"{method_name}(ctx context.Context, request {request_dto_type}) ({response_dto_type}, error)"#
        )
    } else {
        format!(r#"{method_name}(ctx context.Context) error"#)
    };

    method_signature
}

pub fn split_to_pair(s: &str, pattern: &str) -> Option<(String, String)> {
    let mut split = s.split(pattern);

    let first = split.next()?.to_owned();

    let mut second = split.next()?.to_owned();

    while let Some(s) = split.next() {
        second = second.to_string() + s;
    }

    Some((first, second))
}

pub fn find_interface_and_append_method(
    code: &str,
    interface_typename: &str,
    method_code: &str,
) -> String {
    let typename_splited_pair = split_to_pair(code, &interface_typename).unwrap();

    let mut new_content = String::new();

    new_content.push_str(typename_splited_pair.0.as_str());
    new_content.push_str(&interface_typename);

    let brace_splited_pair = split_to_pair(&typename_splited_pair.1, "}").unwrap();
    new_content.push_str(brace_splited_pair.0.as_str());
    new_content.push_str(format!("    {}\n", method_code).as_str());
    new_content.push_str("}");
    new_content.push_str(brace_splited_pair.1.as_str());

    new_content
}

pub fn generate_usecase_method(
    method_name: String,
    has_response: bool,
    config_file: &RootConfig,
) -> String {
    let method_name = method_name.as_str().to_case(Case::Pascal);

    let request_dto_type = generate_request_dto_type(method_name.clone(), config_file);

    let method_signature = if has_response {
        let response_dto_type = generate_response_dto_type(method_name.clone(), config_file);

        format!(
            r#"{method_name}(ctx context.Context, request {request_dto_type}) ({response_dto_type}, error)"#
        )
    } else {
        format!(r#"func () {method_name}(ctx context.Context) error"#)
    };

    method_signature
}
