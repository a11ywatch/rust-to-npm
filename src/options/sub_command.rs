use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Build the project for node.
    BUILD {
        /// custom npm package name ex: @myorg/project
        #[clap(short, long)]
        npm_package_name: Option<String>
    },
    /// Build and deploy the project to crates.io and npm.
    DEPLOY {
        /// build the package for node
        #[clap(short, long)]
        build: bool,
        /// custom npm package name ex: @myorg/project
        #[clap(short, long)]
        npm_package_name: Option<String>
    },
}
