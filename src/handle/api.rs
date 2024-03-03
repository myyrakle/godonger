use super::create_files::{
    create_domain_file_if_not_exists, create_handler_file_if_not_exists,
    create_usecase_file_if_not_exists,
};
use super::edit_files::{
    add_method_to_usecase_interface_of_domain_file, add_method_to_usecase_of_usecase_file,
    add_usecase_interface_to_domain_file_if_not_exists,
};

pub fn new_api(domain: String, api_path: String, method_name: String) {
    // 1. create domain file if not exists
    create_domain_file_if_not_exists(domain.clone());

    // 2. add usecase interface to domain file if not exists
    add_usecase_interface_to_domain_file_if_not_exists(domain.clone());

    // 3. create handler file if not exists
    create_handler_file_if_not_exists(domain.clone());

    // 4. create usecase file if not exists
    create_usecase_file_if_not_exists(domain.clone());

    // 5. add method to usecase interface of domain file
    add_method_to_usecase_interface_of_domain_file(domain.clone(), method_name.clone());

    // 6. add method to usecase file
    add_method_to_usecase_of_usecase_file(&domain, &method_name);

    // 7. add method to handler file
}
