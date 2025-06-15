mod helpers;

#[cfg(test)]
mod extract_all_course_urls_test {
    use rust_sut_courses_api::infrastructure::scraper::courses_data_scraper::extract_all_course_urls;

    use crate::helpers::load_mock_html;

    #[test]
    fn test_extract_all_course_urls() {
        let mock_html = load_mock_html("mock_html_course_table_523*.txt");

        let result = extract_all_course_urls(&mock_html);

        assert_eq!(result.len(), 13);
    }
}
