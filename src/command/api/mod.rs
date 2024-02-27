pub mod new;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "API-related commands")]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    New(new::Command),
}
