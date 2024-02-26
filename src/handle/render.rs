use std::path::PathBuf;

use crate::utils::get_config_file_or_warn;

use super::lookup::InternalFiles;

pub fn render_domain_files(domain_files: &Vec<PathBuf>) -> String {
    let config_file = get_config_file_or_warn();

    let domain_files: Vec<String> = domain_files
        .into_iter()
        .map(|path| {
            let path: PathBuf = path.components().skip(2).collect();
            path.to_str().unwrap().to_string()
        })
        .collect();

    let mut domain_files_text = String::new();
    domain_files_text.push_str(&format!("{}\n", config_file.domain_dir.to_str().unwrap()));
    for domain_file in domain_files {
        domain_files_text.push_str(&format!("|-- {}\n", domain_file));
    }

    domain_files_text
}

pub fn render_internal_files(internal_files: &InternalFiles) -> String {
    let mut internal_files_text = String::new();
    internal_files_text.push_str(&format!("{}\n", internal_files.dir.to_str().unwrap()));

    if let Some(route) = &internal_files.route {
        internal_files_text.push_str(&format!("|-- {}\n", route.dir.to_str().unwrap()));

        for route_detail in &route.details {
            internal_files_text
                .push_str(&format!("|   |-- {}\n", route_detail.dir.to_str().unwrap()));
            for filename in &route_detail.filenames {
                internal_files_text.push_str(&format!("|   |   |-- {}\n", filename));
            }
        }
    }

    if let Some(usecase) = &internal_files.usecase {
        internal_files_text.push_str(&format!("|-- {}\n", usecase.dir.to_str().unwrap()));
        for filename in &usecase.filenames {
            internal_files_text.push_str(&format!("|   |-- {}\n", filename));
        }
    }

    if let Some(store) = &internal_files.store {
        internal_files_text.push_str(&format!("|-- {}\n", store.dir.to_str().unwrap()));

        for store_detail in &store.details {
            internal_files_text
                .push_str(&format!("|   |-- {}\n", store_detail.dir.to_str().unwrap()));
            for filename in &store_detail.filenames {
                internal_files_text.push_str(&format!("|   |   |-- {}\n", filename));
            }
        }
    }

    if let Some(helper) = &internal_files.helper {
        internal_files_text.push_str(&format!("|-- {}\n", helper.dir.to_str().unwrap()));
        for filename in &helper.filenames {
            internal_files_text.push_str(&format!("|   |-- {}\n", filename));
        }
    }

    internal_files_text
}
