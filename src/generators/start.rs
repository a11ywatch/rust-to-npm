/// create the uninstall script for the repo
pub fn generate_start(name: &String) -> String  {
    format!(r#"#!/usr/bin/env node

const {{ exec }} = require("child_process");
const path = require("path");

const binp = path.join(__dirname, "target", "release", "{name}");

exec(binp, (error, stdout, stderr) => {{
  stdout && console.log(stdout);
  stderr && console.log(stderr);
  if (error !== null) {{
    console.log(`exec error: ${{error}}`);
  }}
}});    
    "#)
}