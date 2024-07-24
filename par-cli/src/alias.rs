pub fn apply_aliases(args: &mut Vec<String>) {
    if let Some(second) = args.get(1) {
        if second == "c" || second == "copy" {
            args[1] = "get".to_string();
            args.insert(2, "-c".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let mut args = vec!["par".to_string()];
        apply_aliases(&mut args);
        assert_eq!(vec!["par".to_string()], args);

        args = vec!["par".to_string(), "get".to_string(), "c".to_string()];
        apply_aliases(&mut args);
        assert_eq!(
            vec!["par".to_string(), "get".to_string(), "c".to_string()],
            args
        );

        args = vec!["par".to_string(), "c".to_string(), "key".to_string()];
        apply_aliases(&mut args);
        assert_eq!(
            vec![
                "par".to_string(),
                "get".to_string(),
                "-c".to_string(),
                "key".to_string()
            ],
            args
        );
    }
}
