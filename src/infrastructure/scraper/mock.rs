use crate::presentation::dto::course_response::{CourseDetails, ExamInfo};

pub fn mock_course_details() -> CourseDetails {
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
