use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptionsRun {}

#[derive(Clone, Debug, Args)]
#[clap(name = "gen")]
pub struct GenCommand {
    #[clap(flatten)]
    pub value: ConfigOptionsRun,
}
