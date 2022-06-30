use crate::Package;
use convert_case::{Case, Casing};

/// create the package json matching specs for repo
pub fn generate_package_json(package: &mut Package) -> String {
    let name: String = package.name.to_case(Case::Kebab);
    let keywords = package.keywords.join(",");

    format!(r#"
{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "main": "start.js",
  "directories": {{
    "test": "tests"
  }},
  "scripts": {{
    "test": "echo \"Error: no test specified\" && exit 1",
    "postinstall": "node ./pre-install.js",
    "uninstall": "node ./uninstall.js"
  }},
  "repository": {{
    "type": "git",
    "url": "{}"
  }},
  "keywords": [{}],
  "author": "{}",
  "license": "{}",
  "bugs": {{
    "url": "{}/issues"
  }},
  "homepage": "{}",
  "files": [
    "pre-install.js",
    "start.js",
    "uninstall.js",
    "README.md",
    "LICENSE"
  ]
}}
    "#, 
    name,
    package.version,
    package.description,
    package.repository,
    keywords,
    package.author,
    package.license,
    package.repository,
    package.homepage
  )

}