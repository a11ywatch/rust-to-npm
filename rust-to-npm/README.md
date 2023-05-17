# rust-to-npm

The Rust crate for rust-to-npm.

## Methods

```rust
use rust_to_npm:create_package;

fn main() {
    // colocates a package.json that matches Cargo.toml 1:1
    let package = create_package(None, "Cargo.toml");

    println!("{:?}", package);
}
```

Output:

```json
{
  "name": "rust-to-npm",
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
    "url": "https://github.com/a11ywatch/rust-to-npm.git"
  },
  "keywords": ["rust-to-npm", "deploy-rust", "npm", "rust"],
  "author": "Jeff Mendez",
  "license": "MIT",
  "bugs": {
    "url": "https://github.com/a11ywatch/rust-to-npm/issues"
  },
  "homepage": "https://github.com/a11ywatch/rust-to-npm",
  "files": [
    "pre-install.js",
    "start.js",
    "uninstall.js",
    "README.md",
    "LICENSE"
  ]
}
```