mod helpers;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use rust_sut_courses_api::{
        infrastructure::scraper::courses_data_scraper::scrape_course_data,
        presentation::dto::course_response::{
            ClassSchedule, CourseDetails, CourseName, Exam, ExamInfo, GroupedCourse, Seat, Section,
        },
    };

    use crate::helpers::load_mock_html;

    fn create_mock_course_details() -> CourseDetails {
        CourseDetails {
            course_status: String::from("ใช้งาน"),
            course_condition: Some(vec![String::from("523331")]),
            continue_course: Some(vec![String::from("523435")]),
            equivalent_course: Some(vec![]),
            mid_exam: Some(ExamInfo {
                date: String::from("9"),
                month: String::from("December"),
                times: String::from("12:00-14:00"),
                year: String::from("2567"),
                room: String::from("อาคารB2ห้องB5204(สอบตามตารางมหาวิทยาลัย)"),
            }),
            final_exam: Some(ExamInfo {
                date: String::from("27"),
                month: String::from("January"),
                times: String::from("09:00-11:00"),
                year: String::from("2568"),
                room: String::from("อาคารBห้องB2102(สอบตามตารางมหาวิทยาลัย)"),
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
                en: "SOFTWARE ENGINEERING".to_string(),
                th: Some("วิศวกรรมซอฟต์แวร์".to_string()),
            },
            credit: "4 (3-3-9)".to_string(),
            degree: "1".to_string(),
            department: "วิศวกรรมคอมพิวเตอร์".to_string(),
            faculty: "สำนักวิชาวิศวกรรมศาสตร์".to_string(),
            course_status: "ใช้งาน".to_string(),
            course_condition: mock_course_details.course_condition,
            continue_course: mock_course_details.continue_course,
            equivalent_course: mock_course_details.equivalent_course,
            sections_count: 2,
            sections: mock_sections,
        }]
    }

    #[tokio::test]
    async fn test_scrape_course_data() {
        let mock_html = load_mock_html("mock_html_course_table.txt");

        let result = match scrape_course_data(&mock_html).await {
            Ok(data) => data,
            Err(e) => panic!("Failed to scrape course data: {:?}", e),
        };

        let expect = create_expect_value();

        // Assertions
        assert_eq!(result.len(), 1);
        assert_eq!(result, expect);
    }
}
