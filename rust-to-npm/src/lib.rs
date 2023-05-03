extern crate serde_derive;
extern crate toml;

pub mod generators;

use crate::generators::{package, pre_install, start, uninstall};

use std::env;
use std::fs::OpenOptions;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Default)]
/// package def for npm from cargo
pub struct Package {
    /// the package name
    pub name: String,
    /// the version of the package
    pub version: String,
    /// what the package is about.
    pub description: Option<String>,
    /// github repo url
    pub repository: Option<String>,
    /// keywords for package
    pub keywords: Option<Vec<String>>,
    /// the authors of the package
    pub authors: Option<Vec<String>>,
    /// the package home page
    pub homepage: Option<String>,
    /// the license type
    pub license: Option<String>,
    /// publish the package to crates.io and npm?
    pub publish: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
/// the Cargo.toml definition
pub struct CargoToml {
    /// the package
    pub package: Package,
}

/// create or get a file ready for writting.
pub fn ready_write_file(name: &str) -> File {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&name)
        .expect("Unable to open {name}")
}

/// convert Cargo.toml to Package structure
pub fn get_package_def() -> Package {
    let cargo_toml = format!("{}{}", env::current_dir().unwrap().display(), "/Cargo.toml");
    let cargo_toml = Path::new(&cargo_toml);

    let cargo_file: String = read_to_string(cargo_toml).unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_file).unwrap();
    let package_def: Package = cargo_toml.package;

    package_def
}

/// create npm package contents based off Cargo.toml
pub fn create_package(npm_package_name: &Option<String>, source: &bool) -> Package {
    println!("Building contents...");
    // npm package scripts
    let package_json = "./package.json";
    let pre_install = "./pre-install.js";
    let uninstall = "./uninstall.js";
    let start = "./start.js";

    let mut package_json_file = ready_write_file(&package_json);
    let mut pre_install_file = ready_write_file(&pre_install);
    let mut start_file = ready_write_file(&start);
    let mut uninstall_file = ready_write_file(&uninstall);

    let cargo_toml = format!("{}{}", env::current_dir().unwrap().display(), "/Cargo.toml");
    let cargo_toml = Path::new(&cargo_toml);

    let cargo_file: String = read_to_string(cargo_toml).unwrap();
    let cargo_toml: CargoToml = toml::from_str(&cargo_file).unwrap();
    let mut package_def: Package = cargo_toml.package;

    let name = package_def.name.clone();
    let pkg_name = npm_package_name.as_ref().unwrap_or(&name);

    package_def.name = pkg_name.to_string();

    package_json_file
        .write_all(&package::generate_package_json(&mut package_def, source).as_bytes())
        .unwrap();
    pre_install_file
        .write_all(
            &pre_install::generate_pre_install(&name, &package_def.version, source).as_bytes(),
        )
        .unwrap();
    start_file
        .write_all(&start::generate_start(&name).as_bytes())
        .unwrap();
    uninstall_file
        .write_all(&uninstall::generate_uninstall(&name).as_bytes())
        .unwrap();

    package_json_file.flush().unwrap();
    start_file.flush().unwrap();
    pre_install_file.flush().unwrap();
    uninstall_file.flush().unwrap();

    println!("Finished creating modules for package.");

    package_def
}
