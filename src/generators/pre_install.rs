pub use super::generate_installs;

/// create the pre-install script for the repo
pub fn generate_pre_install(name: &String, version: &String) -> String {
    format!(
        r#"{}
const binp = path.join(cargoDir, "bin", "{name}");

console.log("Installing and compiling {name} {version}...");
exec(`cargo install {name} {version}`, (error, stdout, stderr) => {{
  console.log(stdout);
  if (error || stderr) {{
    console.log(error || stderr);
  }} else {{
    console.log("install finished!");
  }}
}});

    "#,
        generate_installs()
    )
}
