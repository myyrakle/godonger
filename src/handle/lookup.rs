use std::path::PathBuf;

use crate::utils::get_config_file_or_warn;

pub fn lookup_domain(domain: String) -> Vec<PathBuf> {
    let config_file = get_config_file_or_warn();

    let domain_path = config_file.domain_dir;

    let main_file = domain.clone() + ".go";
    let args_file = domain.clone() + &config_file.args_file_suffix + ".go";
    let dto_file = domain.clone() + &config_file.dto_file_suffix + ".go";
    let model_file = domain + &config_file.model_file_suffix + ".go";

    let domain_file_path_list = if !domain_path.exists() {
        vec![]
    } else {
        domain_path
            .read_dir()
            .expect("Failed to read domain directory")
            .map(|entry| entry.expect("Failed to read entry").path())
            .filter(|path| {
                path.ends_with(&main_file)
                    || path.ends_with(&args_file)
                    || path.ends_with(&dto_file)
                    || path.ends_with(&model_file)
            })
            .collect()
    };

    domain_file_path_list
}

pub struct InternalFiles {
    dir: PathBuf,
    route: Option<RouteFiles>,
    usecase: Option<UsecaseFiles>,
    store: Option<StoreFiles>,
    helper: Option<HelperFiles>,
}

impl Default for InternalFiles {
    fn default() -> Self {
        InternalFiles {
            dir: PathBuf::new(),
            route: None,
            usecase: None,
            store: None,
            helper: None,
        }
    }
}

pub struct RouteFiles {
    dir: PathBuf,
    details: Vec<RouteFilesDetail>,
}

pub struct RouteFilesDetail {
    dir: PathBuf,
    filenames: Vec<String>,
}

pub struct StoreFiles {
    dir: PathBuf,
    details: Vec<StoreFilesDetail>,
}

pub struct StoreFilesDetail {
    dir: PathBuf,
    filenames: Vec<String>,
}

pub struct UsecaseFiles {
    dir: PathBuf,
    filenames: Vec<String>,
}

pub struct HelperFiles {
    dir: PathBuf,
    filenames: Vec<String>,
}

pub fn lookup_internal(domain: String) -> InternalFiles {
    let mut internal_files = InternalFiles::default();

    let config_file = get_config_file_or_warn();

    let internal_path = config_file.internal_dir;

    let route_path = internal_path.join(&config_file.route_dir);
    let usecase_path = internal_path.join(&config_file.usecase_dir);
    let store_path = internal_path.join(&config_file.store_dir);
    let helper_path = internal_path.join(&config_file.helper_dir);

    if !internal_path.exists() {
        panic!("Internal directory not found")
    }
    internal_files.dir = internal_path;

    if route_path.exists() {
        let route_files = RouteFiles {
            dir: config_file.route_dir,
            details: vec![],
        };

        internal_files.route = Some(route_files);
    }

    internal_files
}
