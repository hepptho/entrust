use std::path::Path;
use std::process::Command;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    println!("cargo::rerun-if-changed=../.git/HEAD");
    println!("cargo::rerun-if-changed=../.git/refs/heads");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("mod_build_info.rs");
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    let commit = String::from_utf8(output.stdout).unwrap();
    let commit = commit.trim();
    let file_content = format!(
        r#"
pub(crate) mod build_info {{
    pub const HASH: &str = "{commit}";
}}
"#
    );
    fs::write(out_file, file_content)?;
    Ok(())
}
