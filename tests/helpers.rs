use std::fs;

pub fn load_mock_html(file_name: &str) -> String {
    fs::read_to_string(format!("tests/mocks/html/{}", file_name))
        .expect(&format!("Failed to read mock HTML file {}", file_name))
}
