pub(crate) mod wordlist;

#[cfg(test)]
mod tests {
    use crate::generated::wordlist::WORDLIST;
    use itertools::Itertools;

    #[test]
    fn test_wordlist() {
        let unique: Vec<_> = WORDLIST.iter().unique().collect();
        let len = unique.len();
        assert!(
            len > 5000,
            "wordlist should have > 5000 unique entries; has only {}",
            len
        );
    }
}
