use crate::Package;
use convert_case::{Case, Casing};

/// create the package json matching specs for repo
pub fn generate_package_json(package: &mut Package, source: &bool) -> String {
    let name: String = package.name.to_case(Case::Kebab);
    let keywords = package
        .keywords
        .clone()
        .unwrap_or(Vec::new())
        .iter()
        .map(|word| format!(r#""{}""#, word))
        .collect::<Vec<String>>()
        .join(",");
    let authors = package.authors.clone().unwrap_or(Vec::new());
    let license = package.license.clone().unwrap_or("ISC".to_string());
    let repository = package.repository.clone().unwrap_or("".to_string());
    let homepage = package.homepage.clone().unwrap_or("".to_string());
    let description = package.description.clone().unwrap_or("".to_string());

    let authors = if authors.is_empty() {
        "".to_string()
    } else {
        let first = authors.first().unwrap();
        format!(r#""author": "{}","#, first)
    };

    let bugs = if repository.is_empty() {
        "".to_string()
    } else {
        format!(
            r#",
  "bugs": {{
    "url": "{}/issues"
  }}"#,
            repository
        )
    };

    let repository = if repository.is_empty() {
        "".to_string()
    } else {
        format!(
            r#"
  "repository": {{
    "type": "git",
    "url": "{}.git"
  }},"#,
            repository
        )
    };

    let description = if description.is_empty() {
        println!("Description is optional on the Cargo.toml, but nice to add to let others know what the package is about.");
        "".to_string()
    } else {
        format!(r#""description": "{}","#, description)
    };

    let files = if source == &true {
        ""
    } else {
        r#",
  "files": [
    "pre-install.js",
    "start.js",
    "uninstall.js",
    "README.md",
    "LICENSE"
  ]"#
    };

    let homepage = if homepage.is_empty() {
        "".to_string()
    } else {
        format!(
            r#",
  "homepage": "{}""#,
            homepage
        )
    };

    let license = if license.is_empty() {
        "".to_string()
    } else {
        format!(
            r#"
  "license": "{}""#,
            license
        )
    };

    format!(
        r#"{{
  "name": "{}",
  "version": "{}",
  {}
  "main": "start.js",
  "directories": {{
    "test": "tests"
  }},
  "scripts": {{
    "test": "echo \"Error: no test specified\" && exit 1",
    "postinstall": "node ./pre-install.js",
    "uninstall": "node ./uninstall.js"
  }},{}
  "keywords": [{}],
  {}{}{}{}{}
}}
    "#,
        name,
        package.version,
        description,
        repository,
        keywords,
        authors,
        license,
        bugs,
        homepage,
        files
    )
}
