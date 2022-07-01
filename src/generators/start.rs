/// create the uninstall script for the repo
pub fn generate_start(name: &String) -> String  {
    format!(r#"#!/usr/bin/env node

const {{ exec }} = require("child_process");

exec("{name}", (error, stdout, stderr) => {{
  stdout && console.log(stdout);
  stderr && console.log(stderr);
  if (error !== null) {{
    console.log(`exec error: ${{error}}`);
  }}
}});    
    "#)
}