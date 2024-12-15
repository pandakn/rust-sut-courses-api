use regex::Regex;

/// Extracts the value enclosed in parentheses from a given string.
///
/// This function searches for the first pair of parentheses in the input string
/// and returns the trimmed value inside them. If no parentheses are found,
/// or if the parentheses are empty, it returns `None`.
///
/// # Arguments
/// - `input`: A string slice that may contain a value inside parentheses.
///
/// # Returns
/// - `Some(String)` containing the trimmed value inside parentheses, if found.
/// - `None` if no parentheses or content are found.
///
/// # Examples
/// ```
/// use rust_sut_courses_api::utils::string_utils::extract_value_in_brackets;
///
/// let input = "The price is (100 USD)";
/// assert_eq!(extract_value_in_brackets(input), Some("100 USD".to_string()));
///
/// let no_brackets = "No parentheses here";
/// assert_eq!(extract_value_in_brackets(no_brackets), None);
/// ```
pub fn extract_value_in_brackets(input: &str) -> Option<String> {
    if let Some(start) = input.find('(') {
        if let Some(end) = input.find(')') {
            let value = &input[(start + 1)..end];
            return Some(value.trim().to_string());
        }
    }
    None
}

/// Removes extra spaces from a string and ensures single-space separation.
///
/// This function trims leading and trailing whitespace from the input string
/// and replaces any sequence of whitespace characters (spaces, tabs, etc.)
/// with a single space.
///
/// # Arguments
/// - `s`: A string slice to process for excess spaces.
///
/// # Returns
/// - A `String` with excess spaces removed and single-space separation.
///
/// # Examples
/// ```
/// use rust_sut_courses_api::utils::string_utils::trim_space;
///
/// let input = "  Hello   world!   ";
/// assert_eq!(trim_space(input), "Hello world!");
///
/// let empty_input = "   ";
/// assert_eq!(trim_space(empty_input), "");
/// ```
pub fn trim_space(s: &str) -> String {
    Regex::new(r"\s+")
        .unwrap()
        .replace_all(s.trim(), " ")
        .to_string()
}
