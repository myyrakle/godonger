use crate::{
    command::init::ConfigOptions,
    config::{self, RootConfig},
};

pub fn run(option: ConfigOptions) {
    if config::exists_config() {
        eprintln!("Config file already exists");
        std::process::exit(1);
    }

    if option.default {
        let default = RootConfig::default();
        default.write();
    } else {
        unimplemented!("Custom settings not implemented")
    }
}
