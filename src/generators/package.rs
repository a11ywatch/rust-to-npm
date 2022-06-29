use crate::Package;
use convert_case::{Case, Casing};

/// create the package json matching specs for repo
pub fn generate_package_json(package: &mut Package) -> String {
    let name: String = package.name.to_case(Case::Kebab);

    // TODO: get keywords
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
  "license": "MIT",
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
    package.git_url,
    keywords,
    package.author,
    package.git_url,
    package.homepage
  )

}