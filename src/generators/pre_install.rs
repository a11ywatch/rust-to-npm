pub use super::generate_installs;

/// create the pre-install script for the repo
pub fn generate_pre_install(name: &String, version: &String, source: &bool) -> String {
    let install_cmd = if source == &true {
        format!(r#"`cargo install --path ./node_modules/{name} --vers {version} ${{features}}`"#)
    } else {
        format!(r#"`cargo install {name} --vers {version} ${{features}}`"#)
    };

    format!(
        r#"{}
const features = process.env.npm_config_features ? `--features ${{process.env.npm_config_features.replace(",", " ")}}` : ""; 

console.log(`Installing and compiling {name} {version} ${{features}} ...`);
exec({install_cmd}, (error, stdout, stderr) => {{
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
