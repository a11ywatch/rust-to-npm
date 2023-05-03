# rust-to-npm-cli

Deploy a Rust project to crates.io and npm in one command.

https://user-images.githubusercontent.com/8095978/176904499-ef3f5508-1143-491a-bf76-8f240e8b61fb.mov

## Getting Started

Convert and deploy a [Rust](https://www.rust-lang.org) application into a npm package that can be installed on all systems by leveraging cargo.

You need to have a valid `license` set if you want to deploy to crates.io (we may auto fill it in the future).

A valid C compiler is also required to build the crate. By default most linux distros and mac have one installed.

Example on ChromeOS:

```sh
sudo apt update && apt upgrade
sudo apt install build-essential software-properties-common gcc g++
```

You may want to add the following to your `.gitignore` and just reference the project being made with.

```sh
package.json
start.js
uninstall.js
pre-install.js
```

## Installation

Installs can be handled with cargo or npm using the following:

```sh
# install with cargo
cargo install rust-to-npm-cli
# install with npm
npm i rust-to-npm-cli -g
```

You can also use the crate directly at [rust-to-npm](./rust-to-npm/) with `cargo install rust-to-npm`.

## Usage

Run the command inside the repo with the Cargo.toml file to deploy to [crates.io](https://crates.io/) and [npm](https://www.npmjs.com/).

```sh
# to build locally without deploying
rust-to-npm-cli build
# to build and deploy to cargo + npm
rust-to-npm-cli deploy -b
```

You can also pass in the `-n` option on `build & deploy` in order to use a custom npm package name ex: `rust-to-npm-cli build -n @myorg/rust-to-npm-cli`

### Consuming

To use the node_module you just published run `npm i $PACKAGE_NAME --features=somefeature,nextfeature`.

Then use it via node `node node_modules/$PACKAGE_NAME/start.js`. Replace `$PACKAGE_NAME` with your package name as kebab case.
You can also import the module directly at root as a normal module in js like `require("node_modules/$PACKAGE_NAME")`.
The features arg is an optional comma seperated list to use for cargo.

## Goals

1. Easy convert installs to `npm`.
1. Allows full transparency of Rust code that user installs instead of dangerous binaries.

## Steps on how it works

1. convert Cargo.toml to package.json equivalent.
1. `pre-install` script that builds the rust project to the OS.
1. `start` script that executes the binary easily in nodejs.
1. `uninstall` script that removes the binary.

Here is an example of the output from the Cargo.toml to package.json file in the repo.

```toml
[package]

name = "rust-to-npm-cli"
version = "0.4.10"
edition = "2021"
description = "ship a rust project to npm on all operating systems leveraging cargo."
repository = "https://github.com/a11ywatch/rust-to-npm"
readme = "README.md"
keywords = ["rust-to-npm-cli", "deploy-rust", "npm", "rust"]
categories = ["command-line-utilities", "development-tools::build-utils"]
license = "MIT"
homepage = "https://github.com/a11ywatch/rust-to-npm"
authors = ["Jeff Mendez"]

[dependencies]

serde = { version = "1", features=["derive"]}
serde_derive = "1.0.137"
toml = "0.5.9"
```

```json
{
  "name": "rust-to-npm-cli",
  "version": "0.4.10",
  "description": "ship a rust project to npm on all operating systems leveraging cargo.",
  "main": "start.js",
  "directories": {
    "test": "tests"
  },
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1",
    "postinstall": "node ./pre-install.js",
    "uninstall": "node ./uninstall.js"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/a11ywatch/rust-to-npm-cli.git"
  },
  "keywords": ["rust-to-npm-cli", "deploy-rust", "npm", "rust"],
  "author": "Jeff Mendez",
  "license": "MIT",
  "bugs": {
    "url": "https://github.com/a11ywatch/rust-to-npm-cli/issues"
  },
  "homepage": "https://github.com/a11ywatch/rust-to-npm-cli",
  "files": [
    "pre-install.js",
    "start.js",
    "uninstall.js",
    "README.md",
    "LICENSE"
  ]
}
```

## Options

Run the CLI with `--npm_package_name` in order to add a custom package name for npm to use ( good for org packages ).
Use `--source` for shipping and compiling the project from source code.

## About

This project is used to convert the [A11yWatch CLI](https://github.com/a11ywatch/a11ywatch) for usage in node.

See example usage converting the [rust crawler](https://github.com/a11ywatch/crawler) library as a sidecar into a [node project](https://github.com/a11ywatch/sidecar).