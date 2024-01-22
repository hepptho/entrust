use std::{fs, io};

const IN: &str = "eff_large.wordlist";
const OUT: &str = "src/generated/wordlist.rs";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={IN}");
    println!("cargo:rerun-if-changed={OUT}");
    let wordlist = fs::read_to_string(IN)?;
    let wordlist: Vec<_> = wordlist.lines().collect();
    let file_content = format!(
        "pub(crate) const WORDLIST: [&str; {}] = {:#?};\n",
        wordlist.len(),
        wordlist,
    );
    fs::write(OUT, file_content)?;
    Ok(())
}
