#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    use rust_sut_courses_api::{
        infrastructure::scraper::courses_data_scraper::scrape_course_data,
        presentation::dto::course_response::{
            ClassSchedule, CourseDetails, CourseName, Exam, ExamInfo, GroupedCourse, Seat, Section,
        },
    };

    fn create_mock_course_details() -> CourseDetails {
        CourseDetails {
            course_status: String::from("Active"),
            course_condition: Some(vec![
                String::from("Must have passed Math 101"),
                String::from("Must be in the 2nd year"),
            ]),
            continue_course: Some(vec![String::from("Advanced Math 201")]),
            equivalent_course: Some(vec![String::from("Math 101B"), String::from("Math 101C")]),
            mid_exam: Some(ExamInfo {
                date: String::from("10"),
                month: String::from("June"),
                times: String::from("10:00 AM"),
                year: String::from("2024"),
                room: String::from("Room 101"),
            }),
            final_exam: Some(ExamInfo {
                date: String::from("20"),
                month: String::from("June"),
                times: String::from("02:00 PM"),
                year: String::from("2024"),
                room: String::from("Room 202"),
            }),
        }
    }

    fn create_mock_sections() -> Vec<Section> {
        let mock_course_details = create_mock_course_details();

        vec![
            Section {
                id: "a3e19ddc-6ba1-404e-8603-b14b59bbbe4c".to_string(),
                url: "mock url".to_string(),
                section: "1".to_string(),
                status: "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้".to_string(),
                note: Some("This is mock note".to_string()),
                professors: vec![
                    "Dr. AA AA".to_string(),
                    "Dr. BB BB".to_string(),
                    "Dr. CC CC".to_string(),
                    "Dr. DD DD".to_string(),
                ],
                language: "TH".to_string(),
                seat: Seat {
                    total_seat: "5".to_string(),
                    registered: "5".to_string(),
                    remain: "0".to_string(),
                },
                class_schedule: vec![
                    ClassSchedule {
                        day: "Mo".to_string(),
                        times: "09:00-12:00".to_string(),
                        room: Some("B1121".to_string()),
                    },
                    ClassSchedule {
                        day: "We".to_string(),
                        times: "09:00-12:00".to_string(),
                        room: Some("F11-422.Software".to_string()),
                    },
                ],
                exams: Exam {
                    midterm: mock_course_details.mid_exam.clone(),
                    r#final: mock_course_details.final_exam.clone(),
                },
            },
            Section {
                id: "56f1c1f7-7bee-452b-92a4-4afba710f475".to_string(),
                url: "mock url".to_string(),
                section: "2".to_string(),
                status: "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้".to_string(),
                note: Some("This is mock note".to_string()),
                professors: vec![
                    "Dr. AA AA".to_string(),
                    "Dr. BB BB".to_string(),
                    "Dr. CC CC".to_string(),
                    "Dr. DD DD".to_string(),
                ],
                language: "TH".to_string(),
                seat: Seat {
                    total_seat: "5".to_string(),
                    registered: "5".to_string(),
                    remain: "0".to_string(),
                },
                class_schedule: vec![
                    ClassSchedule {
                        day: "Mo".to_string(),
                        times: "09:00-12:00".to_string(),
                        room: Some("B1121".to_string()),
                    },
                    ClassSchedule {
                        day: "We".to_string(),
                        times: "13:00-16:00".to_string(),
                        room: Some("F11-422.Software".to_string()),
                    },
                ],
                exams: Exam {
                    midterm: mock_course_details.mid_exam.clone(),
                    r#final: mock_course_details.final_exam.clone(),
                },
            },
        ]
    }

    fn create_expect_value() -> Vec<GroupedCourse> {
        let mock_course_details = create_mock_course_details();
        let mock_sections = create_mock_sections();

        vec![GroupedCourse {
            course_code: "523332".to_string(),
            version: "2".to_string(),
            course_name: CourseName {
                en: "mock course name en".to_string(),
                th: Some("mock course name th".to_string()),
            },
            credit: "4 (3-3-9)".to_string(),
            degree: "1".to_string(),
            department: "mock department".to_string(),
            faculty: "mock faculty".to_string(),
            course_status: "Active".to_string(),
            course_condition: mock_course_details.course_condition,
            continue_course: mock_course_details.continue_course,
            equivalent_course: mock_course_details.equivalent_course,
            sections_count: 2,
            sections: mock_sections,
        }]
    }

    #[test]
    fn test_scrape_course_data() {
        // Read the mock HTML content from a file
        let mock_html = fs::read_to_string("tests/mock_html_course_table.txt")
            .expect("Failed to read mock HTML file");

        let result = match scrape_course_data(&mock_html) {
            Ok(data) => data,
            Err(e) => panic!("Failed to scrape course data: {:?}", e),
        };

        let expect = create_expect_value();

        // Assertions
        assert_eq!(result.len(), 1);
        assert_eq!(result, expect);
    }
}
