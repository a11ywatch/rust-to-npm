extern crate clap;
extern crate serde_derive;
extern crate toml;
extern crate rust_to_npm;

pub mod options;

use options::{Cli, Commands};
use std::process::Command;
use clap::Parser;

/// convert a rust project to npm install cross compiled for all systems.
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::BUILD {
            source,
            npm_package_name,
        }) => {
            rust_to_npm::create_package(npm_package_name, source);
        }
        Some(Commands::DEPLOY {
            source,
            build,
            npm_package_name,
        }) => {
            let package_def = if *build {
                rust_to_npm::create_package(npm_package_name, source)
            } else {
                rust_to_npm::get_package_def()
            };

            Command::new("git")
                .args(["add", "."])
                .status()
                .expect("Failed to execute git add command");

            Command::new("git")
                .args([
                    "commit",
                    "-m",
                    &format!("release: build v{}", &package_def.version)[..],
                ])
                .status()
                .expect("Failed to execute git commit command");

            println!("Deploying to crates.io...");
            Command::new("cargo")
                .args(["publish"])
                .status()
                .expect("Failed to execute cargo publish command");

            if package_def.publish.is_some() && package_def.publish.unwrap_or(true) == false {
                println!("package created locally. Publishing will occur if repo is set to private on npm.");
                Command::new("npm")
                    .args(["publish"])
                    .status()
                    .expect("Failed to execute npm publish command");
            } else {
                println!("Deploying to npm...");
                Command::new("npm")
                    .args(["publish", "--access public"])
                    .status()
                    .expect("Failed to execute npm publish command");
            }
        }
        None => {}
    }
}
