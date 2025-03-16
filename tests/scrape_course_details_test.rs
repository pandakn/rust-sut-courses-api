mod helpers;

#[cfg(test)]
mod scrape_course_details_test {
    use pretty_assertions::assert_eq;

    use rust_sut_courses_api::{
        infrastructure::scraper::course_details_scraper::scrape_course_details,
        presentation::dto::course_response::{CourseDetailResponse, ExamInfo},
    };

    use crate::helpers::load_mock_html;

    fn create_expect_value() -> CourseDetailResponse {
        CourseDetailResponse {
            course_name_en: String::from("SOFTWARE ENGINEERING"),
            course_name_th: String::from("วิศวกรรมซอฟต์แวร์"),
            faculty: String::from("สำนักวิชาวิศวกรรมศาสตร์"),
            department: String::from("วิศวกรรมคอมพิวเตอร์"),
            course_status: String::from("ใช้งาน"),
            course_condition: vec![String::from("523331")],
            continue_course: vec![String::from("523435")],
            equivalent_course: vec![],
            midterm: Some(ExamInfo {
                date: String::from("9"),
                month: String::from("December"),
                times: String::from("12:00-14:00"),
                year: String::from("2567"),
                room: String::from("อาคารB2ห้องB5204(สอบตามตารางมหาวิทยาลัย)"),
            }),
            r#final: Some(ExamInfo {
                date: String::from("27"),
                month: String::from("January"),
                times: String::from("09:00-11:00"),
                year: String::from("2568"),
                room: String::from("อาคารBห้องB2102(สอบตามตารางมหาวิทยาลัย)"),
            }),
        }
    }

    #[test]
    fn test_scrape_course_data() {
        let mock_html = load_mock_html("5_mock_html_course_details_full.txt");

        let result = match scrape_course_details(&mock_html) {
            Ok(data) => data.unwrap(),
            Err(e) => panic!("Failed to scrape course details: {:?}", e),
        };

        let expect = create_expect_value();

        // Assertions
        assert_eq!(result, expect);
    }
}
