#[cfg(test)]
mod scrape_course_requirements_test {
    use std::collections::HashSet;
    use std::fs;

    use pretty_assertions::assert_eq;

    use rust_sut_courses_api::{
        infrastructure::scraper::course_details_scraper::scrape_course_requirements,
        presentation::dto::course_response::CourseRequirements,
    };

    fn assert_course_requirements_unordered(
        result: &CourseRequirements,
        expected: &CourseRequirements,
    ) {
        assert_eq!(
            HashSet::<_>::from_iter(result.course_condition.iter()),
            HashSet::<_>::from_iter(expected.course_condition.iter()),
            "course_condition mismatch"
        );
        assert_eq!(
            HashSet::<_>::from_iter(result.continue_course.iter()),
            HashSet::<_>::from_iter(expected.continue_course.iter()),
            "continue_course mismatch"
        );
        assert_eq!(
            HashSet::<_>::from_iter(result.equivalent_course.iter()),
            HashSet::<_>::from_iter(expected.equivalent_course.iter()),
            "equivalent_course mismatch"
        );
    }

    #[test]
    fn test_scrape_course_condition() {
        let mock_html_1 = fs::read_to_string("tests/mocks/1_mock_html_course_details.txt")
            .expect("Failed to read mock HTML file 1");
        let mock_html_2 = fs::read_to_string("tests/mocks/2_mock_html_course_details.txt")
            .expect("Failed to read mock HTML file 2");
        let mock_html_3 = fs::read_to_string("tests/mocks/3_mock_html_course_details.txt")
            .expect("Failed to read mock HTML file 3");
        let mock_html_4 =
            fs::read_to_string("tests/mocks/4_mock_html_course_details_no_requirements.txt")
                .expect("Failed to read mock HTML file 4");

        let expect_1 = CourseRequirements {
            course_condition: vec!["523331".to_string()],
            continue_course: vec!["523435".to_string()],
            equivalent_course: vec![],
        };

        let expect_2 = CourseRequirements {
            course_condition: vec!["523201".to_string(), "ENG23 2001".to_string()],
            continue_course: vec![
                "523453".to_string(),
                "523454".to_string(),
                "523460".to_string(),
            ],
            equivalent_course: vec![],
        };

        let expect_3 = CourseRequirements {
            course_condition: vec!["523490".to_string(), "CWI01 4100".to_string()],
            continue_course: vec!["523492".to_string()],
            equivalent_course: vec!["523491".to_string(), "CWI01 4101".to_string()],
        };

        let expect_4 = CourseRequirements {
            course_condition: vec![],
            continue_course: vec![],
            equivalent_course: vec![],
        };

        let result_1: CourseRequirements = scrape_course_requirements(&mock_html_1).unwrap();
        assert_course_requirements_unordered(&result_1, &expect_1);

        let result_2: CourseRequirements = scrape_course_requirements(&mock_html_2).unwrap();
        assert_course_requirements_unordered(&result_2, &expect_2);

        let result_3: CourseRequirements = scrape_course_requirements(&mock_html_3).unwrap();
        assert_course_requirements_unordered(&result_3, &expect_3);

        let result_4: CourseRequirements = scrape_course_requirements(&mock_html_4).unwrap();
        assert_course_requirements_unordered(&result_4, &expect_4);
    }
}
