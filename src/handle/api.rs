use super::create_files::{
    create_domain_dto_file_if_not_exists, create_domain_file_if_not_exists,
    create_handler_file_if_not_exists, create_usecase_file_if_not_exists,
};
use super::edit_files::{
    add_dto_types_to_domain_file, add_method_to_handler_file,
    add_method_to_usecase_interface_of_domain_file, add_method_to_usecase_of_usecase_file,
    add_usecase_interface_to_domain_file_if_not_exists,
};

pub fn new_api(domain: String, api_path: String, method_name: String, has_response: bool) {
    // 1. create domain file if not exists
    create_domain_file_if_not_exists(domain.clone());

    // 2. create domain dto file if not exists
    create_domain_dto_file_if_not_exists(domain.clone());

    // 3. add DTO typs to domain file
    add_dto_types_to_domain_file(domain.clone(), method_name.clone(), has_response);

    // 4. add usecase interface to domain file if not exists
    add_usecase_interface_to_domain_file_if_not_exists(domain.clone());

    // 5. create handler file if not exists
    create_handler_file_if_not_exists(domain.clone());

    // 6. create usecase file if not exists
    create_usecase_file_if_not_exists(domain.clone());

    // 7. add method to usecase interface of domain file
    add_method_to_usecase_interface_of_domain_file(domain.clone(), method_name.clone());

    // 8. add method to usecase file
    add_method_to_usecase_of_usecase_file(&domain, &method_name);

    // 9. add method to handler file
    add_method_to_handler_file(&domain, &api_path, &method_name, has_response);
}
