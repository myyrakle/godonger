mod command;

use clap::Parser;
use command::SubCommand;

fn main() {
    let args = command::Command::parse();

    match args.action {
        SubCommand::Gen(_gen) => {}
    }
}
