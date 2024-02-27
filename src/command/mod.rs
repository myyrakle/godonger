pub mod api;
pub mod domain;
pub mod init;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    Init(init::Command),
    Domain(domain::Command),
    API(api::Command),
}
