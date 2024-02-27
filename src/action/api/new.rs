use crate::{
    command::api::new::ConfigOptions,
    tui::{enter_tui, exit_tui},
    utils::get_config_file_or_warn,
};

pub fn run(_: ConfigOptions) {
    let config_file = get_config_file_or_warn();

    let mut terminal = enter_tui();
    //crate::tui::domain_list::run(&mut terminal, domain_list).unwrap();
    exit_tui();
}
