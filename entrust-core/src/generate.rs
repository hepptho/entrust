use rand::prelude::IteratorRandom;

const WORDLIST: &str = include_str!("../eff_large.wordlist");

const PRINTABLE_ASCII: &str = r#"!"$#%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#;

pub fn generate_passphrase(len: u8, separator: &str) -> String {
    let phrase_iterable = (0..len).map(|_| random_word());
    itertools::intersperse(phrase_iterable, separator).collect()
}

pub fn generate_password(len: u8) -> String {
    (0..len).map(|_| random_ascii()).collect()
}

fn random_word() -> &'static str {
    WORDLIST.lines().choose(&mut rand::thread_rng()).unwrap()
}

pub fn random_ascii() -> char {
    PRINTABLE_ASCII
        .chars()
        .choose(&mut rand::thread_rng())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_wordlist() {
        let unique: Vec<_> = WORDLIST.lines().unique().collect();
        let len = unique.len();
        assert!(
            len > 5000,
            "wordlist should have > 5000 unique entries; has only {}",
            len
        );
    }
}
