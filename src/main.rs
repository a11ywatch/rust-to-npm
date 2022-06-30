extern crate convert_case;
extern crate toml;
extern crate serde_derive;

pub mod generators;

use crate::generators::{package, pre_install, start, uninstall};
use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::process::Command;
use std::path::Path;
use serde_derive::Deserialize;

#[derive(Deserialize, Default)]
/// package def for npm from cargo
pub struct Package {
    /// the crate name
    name: String,
    /// the version of the crate
    version: String,
    /// description of the crate
    description: String,
    /// github repo url
    repository: String,
    /// keywords for crate
    keywords: Vec<String>,
    /// the author of the crate
    author: String,
    /// the package home page
    homepage: String,
    /// the license type
    license: String,
}

/// create or get a file ready for writting.
fn ready_write_file(name: &str) -> File {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&name)
        .expect("Unable to open {name}")
}

/// convert a rust project to npm install cross compiled for all systems.
fn main() {
    println!("work in progress... issues may exist.");

    let package_json = "./package.json";
    let pre_install = "./pre-install.js";
    let uninstall = "./uninstall.js";
    let start = "./start.js";

    let mut package_json_file = ready_write_file(&package_json);
    let mut pre_install_file = ready_write_file(&pre_install);
    let mut start_file = ready_write_file(&start);
    let mut uninstall_file = ready_write_file(&uninstall);

    let cargo_file: String = read_to_string(Path::new("./Cargo.toml")).unwrap().parse().unwrap();
    let mut package: Package = toml::from_str(&cargo_file).unwrap_or_default();
    let name = package.name.clone();
    
    package_json_file.write_all(&package::generate_package_json(&mut package).as_bytes()).unwrap();
    pre_install_file.write_all(&pre_install::generate_pre_install(&name).as_bytes()).unwrap();
    start_file.write_all(&start::generate_start(&name).as_bytes()).unwrap();
    uninstall_file.write_all(&uninstall::generate_uninstall(&name).as_bytes()).unwrap();

    package_json_file.flush().unwrap();
    start_file.flush().unwrap();
    pre_install_file.flush().unwrap();
    uninstall_file.flush().unwrap();

    // TODO: allow private
    Command::new("npm")
        .args(["publish", "--access public"])
        .status()
        .expect("Failed to execute npm publish command");
}
