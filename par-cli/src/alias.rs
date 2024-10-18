pub fn apply_aliases(args: &mut Vec<String>) {
    match args.get(1).map(|s| s.as_str()) {
        Some("c") | Some("copy") => {
            args[1] = "get".to_string();
            args.insert(2, "-c".to_string())
        }
        Some("a") | Some("t") | Some("type") => {
            args[1] = "autotype".to_string();
        }
        _ => {}
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
