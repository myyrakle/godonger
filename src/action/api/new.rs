use crate::{
    command::api::new::ConfigOptions,
    tui::{self, enter_tui, exit_tui},
};

pub fn run(option: ConfigOptions) {
    let mut terminal = enter_tui();
    tui::new_api::run(&mut terminal, option.domain).unwrap();
    exit_tui();
}
