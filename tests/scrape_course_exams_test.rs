mod helpers;

#[cfg(test)]
mod scrape_course_exams_test {
    use rust_sut_courses_api::{
        infrastructure::scraper::course_details_scraper::scrape_course_exams,
        presentation::dto::course_response::{Exam, ExamInfo},
    };

    use crate::helpers::load_mock_html;

    #[test]
    fn test_scrape_course_exams() {
        let html = load_mock_html("exams/1_mock_html_course_exams.txt");
        let result = scrape_course_exams(&html).unwrap();

        let expected = Exam {
            midterm: Some(ExamInfo {
                date: "9".to_string(),
                month: "December".to_string(),
                times: "12:00-14:00".to_string(),
                year: "2567".to_string(),
                room: "อาคารB2ห้องB5204(สอบตามตารางมหาวิทยาลัย)".to_string(),
            }),
            r#final: Some(ExamInfo {
                date: "27".to_string(),
                month: "January".to_string(),
                times: "09:00-12:00".to_string(),
                year: "2568".to_string(),
                room: "อาคารBห้องN(สอบตามตารางมหาวิทยาลัย)".to_string(),
            }),
        };

        assert_eq!(result, expected);
    }
}
