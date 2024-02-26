mod action;
mod command;
mod config;
mod handle;
mod tui;
mod utils;

use clap::Parser;
use command::{domain, SubCommand};

fn main() {
    let args = command::Command::parse();

    match args.action {
        SubCommand::Init(command) => {
            action::init::run(command.value);
        }
        SubCommand::Domain(command) => match command.action {
            domain::SubCommand::List(command) => {
                action::domain::list::run(command.value);
            }
        },
    }
}
