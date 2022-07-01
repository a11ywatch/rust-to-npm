# rust-to-npm

Deploy a rust project to crates.io and npm in one command.

## Getting Started

This project converts your rust project into a npm package that can be installed on all systems by leveraging cargo.

The Cargo.toml properties map to package.json.

You need to have a valid `name`, `version` and `description` set in your Cargo.toml.

## Installation

```sh
# cargo
cargo install rust-to-npm
```

You may want to add the following to your .gitignore and just reference the project being made with.

```
package.json
package-lock.json
start.js
uninstall.js
pre-install.js
```

## Usage

If you want the package to be private make sure to have `private` set in your Cargo.toml file.

Run the command inside the repo with the Cargo.toml file to deploy to [crates.io](https://crates.io/) and [npm](https://www.npmjs.com/).

```sh
# to build and deploy to cargo + npm
rust-to-npm
# to build locally without deploying
rust-to-npm no-deploy
```

### Consuming

To use the node_module you just published run `npm i $PACKAGE_NAME`.

Then use it via node `node node_modules/$PACKAGE_NAME/start.js`. Replace `$PACKAGE_NAME` with your package name as kebab case.
You can also import the module directly at root as a normal module in js like `require("node_modules/$PACKAGE_NAME")`.

## Goals

1. Easy convert installs to `npm`.
1. Allows full transparency of Rust code that user installs instead of dangerous binaries.

## Steps on how it works

1. `pre-install` script that builds the rust project to the OS.
1. `start` script that executes the binary easily in nodejs.
1. `uninstall` script that removes the binary.
1. [TODO] send all rust files that build the project without need for `crate`.
   We have this working on another project, plan is to make this optional between `crate` installs or local for size.

## About

This project is used to convert the rust [A11yWatch CLI](https://github.com/A11yWatch/a11ywatch) for usage in node.

See example usage converting the [rust crawler](https://github.com/A11yWatch/crawler) library as a sidecar into a [node project](https://github.com/A11yWatch/sidecar).

## TODO

1. Allow include local rust src files for non CLI bins for easy `node_module` imports like the crawler project.
