use clap::{Parser};
use crate::options::sub_command::Commands;

/// deploy rust projects to npm with ease.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// main program commands
    #[clap(subcommand)]
    pub command: Option<Commands>
}
