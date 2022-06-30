pub mod package;
pub mod pre_install;
pub mod start;
pub mod uninstall;

/// start the install scripts
pub fn generate_installs() -> &'static str  {
    r#"#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { exec } = require("child_process");

const cargoDir = path.dirname("$HOME" + ".cargo");

// check if directory exists
if (fs.existsSync(cargoDir)) {
  //   console.log("Cargo found.");
} else {
  const setCargo = 'PATH="/$HOME/.cargo/bin:${PATH}"';
  console.log("Installing deps [cargo].");

  exec(
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && ${setCargo}`,
    (error) => {
      if (error) {
        console.log(
          "curl failed! Curl may not be installed on the OS. View https://curl.se/download.html to install."
        );
        console.log(error);
      }
    }
  );
}
    "#
}

// /// generate .gitignore
// pub fn generate_git_ignore() -> &'static str  {
//     r#".DS_Store
// /target
// package.json
// package-lock.json
// start.js
// uninstall.js
// pre-install.js
//     "#
// }