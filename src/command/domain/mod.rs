pub mod list;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Domain-related commands")]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    List(list::Command),
}
