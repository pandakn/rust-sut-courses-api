mod helpers;

#[cfg(test)]
mod extract_course_base_data_test {
    use rust_sut_courses_api::infrastructure::scraper::courses_data_scraper::extract_course_base_data;

    use crate::helpers::load_mock_html;

    #[tokio::test]
    async fn test_extract_course_base_data_test() {
        let mock_html = load_mock_html("mock_html_course_table_523*.txt");

        let course_base = extract_course_base_data(&mock_html);

        assert_eq!(course_base.len(), 28);
    }
}
