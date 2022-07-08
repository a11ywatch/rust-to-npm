use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Build the project for node.
    BUILD {},
    /// Build and deploy the project to crates.io and npm.
    DEPLOY {},
}
