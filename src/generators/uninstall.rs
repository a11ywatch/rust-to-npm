pub use super::generate_installs;

/// create the uninstall script for the repo
pub fn generate_uninstall(name: &String) -> String {
    format!(
        r#"{}
const binp = path.join(cargoDir, "bin", "{name}");

if (fs.existsSync(binp)) {{
  console.log("Uninstalling {name}...");
  exec(`cargo uninstall a11ywatch_cli`, (error, stdout, stderr) => {{
    console.log(stdout);
    if (error || stderr) {{
      console.log(error || stderr);
    }}
  }});
}} else {{
  console.log("{name} not found skipping!");
}}
    
    "#,
        generate_installs()
    )
}
