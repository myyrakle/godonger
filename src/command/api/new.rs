use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, help = "domain name")]
    pub domain: String,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "new", about = "New API to domain")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
