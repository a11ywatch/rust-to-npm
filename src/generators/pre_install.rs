pub use super::generate_installs;

/// create the pre-install script for the repo
pub fn generate_pre_install(name: &String, version: &String) -> String {
    format!(
        r#"{}
const features = process.env.npm_config_features ? `--features ${{process.env.npm_config_features.replace(",", " ")}}` : ""; 

console.log(`Installing and compiling {name} {version} ${{features}} ...`);
exec(`cargo install {name} --vers {version} ${{features}}`, (error, stdout, stderr) => {{
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
