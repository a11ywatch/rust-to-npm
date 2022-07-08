pub use super::generate_installs;

/// create the pre-install script for the repo
pub fn generate_pre_install(name: &String) -> String {
    format!(
        r#"{}
const binp = path.join(cargoDir, "bin", "{name}");

if (!fs.existsSync(binp)) {{
  console.log("Installing and compiling {name}...");
  exec(`cargo install {name}`, (error, stdout, stderr) => {{
    console.log(stdout);
    if (error || stderr) {{
      console.log(error || stderr);
    }} else {{
      console.log("install finished!");
    }}
  }});
}} else {{
  console.log("{name} found install skipping!");
}}

    "#,
        generate_installs()
    )
}
