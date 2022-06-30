# rust-to-npm

Deploy a rust project to [npm](https://www.npmjs.com/) in one command.

## Getting Started

This project converts your rust project into a npm package that can be installed on all systems by leveraging cargo.

The Cargo.toml properties map to package.json and respects the `private` property for publishing.

## Installation

```sh
# cargo
cargo install rust-to-npm
```

You may want to add the following to the .gitignore and just reference the project being made with.

```
package.json
package-lock.json
start.js
uninstall.js
pre-install.js
```

## Usage

```sh
# run the command inside the repo with the Cargo.toml file
rust-to-npm
```

## Goals

1. Easy convert installs to `npm`.
1. Allows full transparency of Rust code that user installs instead of dangerous binaries.

## About

This project is used to convert the rust [A11yWatch CLI](https://github.com/A11yWatch/a11ywatch) for usage in node.

See example usage converting the [rust crawler](https://github.com/A11yWatch/crawler) library as a sidecar into a [node project](https://github.com/A11yWatch/sidecar).

## TODO

1. Allow include local rust src files for non CLI type bins for easy `node_module` imports like the crawler project.
