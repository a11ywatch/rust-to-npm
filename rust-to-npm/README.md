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