mod extract_value_in_brackets_tests {
    use crate::utils::string_utils::extract_value_in_brackets;

    #[test]
    fn test_extract_value_in_brackets_with_value() {
        let input = "The price is ( 100 USD )";
        let result = extract_value_in_brackets(input);
        assert_eq!(result, Some("100 USD".to_string()));
    }

    #[test]
    fn test_extract_value_in_brackets_no_brackets() {
        let input = "No brackets here";
        let result = extract_value_in_brackets(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_value_in_brackets_empty_value() {
        let input = "The price is ()";
        let result = extract_value_in_brackets(input);
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_extract_value_in_brackets_nested_brackets() {
        let input = "The price is (100 USD (approx))";
        let result = extract_value_in_brackets(input);
        assert_eq!(result, Some("100 USD (approx".to_string()));
    }
}

mod trim_space_tests {
    use crate::utils::string_utils::trim_space;

    #[test]
    fn test_trim_space_multiple_spaces() {
        let input = "  Hello   world!   How are you?  ";
        let result = trim_space(input);
        assert_eq!(result, "Hello world! How are you?");
    }

    #[test]
    fn test_trim_space_leading_and_trailing_spaces() {
        let input = "   Multiple    spaces  in    between   ";
        let result = trim_space(input);
        assert_eq!(result, "Multiple spaces in between");
    }

    #[test]
    fn test_trim_space_no_extra_spaces() {
        let input = "NoExtraSpacesHere";
        let result = trim_space(input);
        assert_eq!(result, "NoExtraSpacesHere");
    }

    #[test]
    fn test_trim_space_empty_string() {
        let input = "   ";
        let result = trim_space(input);
        assert_eq!(result, "");
    }

    #[test]
    fn test_trim_space_single_word() {
        let input = "   Rust   ";
        let result = trim_space(input);
        assert_eq!(result, "Rust");
    }
}
