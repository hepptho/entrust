use std::fs;

const IN: &str = "eff_large.wordlist";
const OUT: &str = "src/generated/wordlist.rs";

fn main() {
    println!("cargo:rerun-if-changed={IN}");
    println!("cargo:rerun-if-changed={OUT}");
    let wordlist = fs::read_to_string(IN).expect("failed to read wordlist");
    let wordlist: Vec<_> = wordlist.lines().collect();
    let mut file_content = format!(
        "pub(crate) const WORDLIST: [&str; {}] = [\n",
        wordlist.len()
    );
    for word in wordlist {
        file_content.push_str("    \"");
        file_content.push_str(word);
        file_content.push_str("\",\n")
    }
    file_content.push_str("\n];");
    fs::write(OUT, file_content).expect("failed to write wordlist");
}
