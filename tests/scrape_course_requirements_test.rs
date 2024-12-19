mod helpers;

#[cfg(test)]
mod scrape_course_requirements_test {
    use pretty_assertions::assert_eq;
    use rust_sut_courses_api::{
        infrastructure::scraper::course_details_scraper::scrape_course_requirements,
        presentation::dto::course_response::CourseRequirements,
    };
    use std::collections::HashSet;

    use crate::helpers::load_mock_html;

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

    struct TestCase {
        mock_html: String,
        expected: CourseRequirements,
    }

    #[test]
    fn test_scrape_course_condition() {
        let test_cases = vec![
            TestCase {
                mock_html: load_mock_html("1_mock_html_course_details.txt"),
                expected: CourseRequirements {
                    course_condition: vec!["523331".to_string()],
                    continue_course: vec!["523435".to_string()],
                    equivalent_course: vec![],
                },
            },
            TestCase {
                mock_html: load_mock_html("2_mock_html_course_details.txt"),
                expected: CourseRequirements {
                    course_condition: vec!["523201".to_string(), "ENG23 2001".to_string()],
                    continue_course: vec![
                        "523453".to_string(),
                        "523454".to_string(),
                        "523460".to_string(),
                    ],
                    equivalent_course: vec![],
                },
            },
            TestCase {
                mock_html: load_mock_html("3_mock_html_course_details.txt"),
                expected: CourseRequirements {
                    course_condition: vec!["523490".to_string(), "CWI01 4100".to_string()],
                    continue_course: vec!["523492".to_string()],
                    equivalent_course: vec!["523491".to_string(), "CWI01 4101".to_string()],
                },
            },
            TestCase {
                mock_html: load_mock_html("4_mock_html_course_details_no_requirements.txt"),
                expected: CourseRequirements {
                    course_condition: vec![],
                    continue_course: vec![],
                    equivalent_course: vec![],
                },
            },
        ];

        for test_case in test_cases.iter() {
            let result: CourseRequirements =
                scrape_course_requirements(&test_case.mock_html).unwrap();
            assert_course_requirements_unordered(&result, &test_case.expected);
        }
    }
}
