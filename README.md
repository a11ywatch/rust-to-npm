# rust-to-npm

rust to npm in one command.

## Getting Started

You can use this to run the command locally then run `npm publish` to ship your crate cross platform leveraging cargo to build across all systems.

<b>Warning!</b>: at the moment it only publishes the project publicly only.

## Installation

```sh
# cargo
cargo install rust-to-npm
# run the command inside the repo with the Cargo.toml file
rust-to-npm
```

You may want to add the following to the .gitignore and just reference the project being made with.

```
.DS_Store
/target
package.json
package-lock.json
start.js
uninstall.js
pre-install.js
```

## Goals

1. Easy convert installs to `npm`.
1. Allows full transparency of Rust code that user installs instead of dangerous binaries.

## About

This project is used to convert the rust [crawler crate](https://github.com/A11yWatch/crawler) for usage in node.
See example project of library being used as a sidecar from rust into a [node project](https://github.com/A11yWatch/sidecar).

## TODO

1. Allow include local rust src files for non CLI type bins for easy `node_module` imports.
