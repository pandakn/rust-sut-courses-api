use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseRequirements {
    pub course_condition: Vec<String>,
    pub continue_course: Vec<String>,
    pub equivalent_course: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseDetailResponse {
    pub course_name_en: String,
    pub course_name_th: String,
    pub faculty: String,
    pub department: String,
    pub course_status: String,
    pub course_condition: Vec<String>,
    pub continue_course: Vec<String>,
    pub equivalent_course: Vec<String>,
    pub midterm: Option<ExamInfo>,
    pub r#final: Option<ExamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTableResponse {
    pub url: String,
    pub course_code: String,
    pub credit: String,
}

/// Represents exam information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExamInfo {
    pub date: String,
    pub month: String,
    pub times: String,
    pub year: String,
    pub room: String,
}

/// Represents midterm and final exam details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exam {
    pub midterm: Option<ExamInfo>,
    pub r#final: Option<ExamInfo>,
}

/// Represents course name in multiple languages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseName {
    pub en: String,
    pub th: Option<String>,
}

/// Represents the class schedule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassSchedule {
    pub day: String,
    pub times: String,
    pub room: Option<String>,
}

/// Represents seat information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Seat {
    pub total_seat: String,
    pub registered: String,
    pub remain: String,
}

/// Represents detailed course information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseDetails {
    pub course_status: String,
    pub course_condition: Option<Vec<String>>,
    pub continue_course: Option<Vec<String>>,
    pub equivalent_course: Option<Vec<String>>,
    pub mid_exam: Option<ExamInfo>,
    pub final_exam: Option<ExamInfo>,
}

/// Represents a course
#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq)]
pub struct Course {
    #[derivative(PartialEq = "ignore")]
    pub id: String,
    pub url: String,
    pub course_code: String,
    pub version: String,
    pub course_name_en: String,
    pub course_name_th: Option<String>,
    pub faculty: String,
    pub department: String,
    pub note: Option<String>,
    pub professors: Vec<String>,
    pub credit: String,
    pub section: String,
    pub status_section: String,
    pub language: String,
    pub degree: String,
    pub class_schedule: Vec<ClassSchedule>,
    pub seat: Seat,
    pub details: CourseDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
pub struct CourseBase {
    pub course_code: String,
    pub version: String,
    pub url: String,
    pub note: Option<String>,
    pub professors: Vec<String>,
    pub credit: String,
    pub section: String,
    pub status_section: String,
    pub language: String,
    pub degree: String,
    pub class_schedule: Vec<ClassSchedule>,
    pub seat: Seat,
    // pub details: CourseDetails,
}

/// Represents a section of a course
#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq)]
pub struct Section {
    #[derivative(PartialEq = "ignore")]
    pub id: String,

    #[derivative(PartialEq = "ignore")]
    pub url: String,
    pub section: String,
    pub status: String,
    pub note: Option<String>,
    pub professors: Vec<String>,
    pub language: String,
    #[derivative(PartialEq = "ignore")]
    pub seat: Seat,
    pub class_schedule: Vec<ClassSchedule>,
    pub exams: Exam,
}

/// Represents a grouped course structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupedCourse {
    pub course_code: String,
    pub version: String,
    pub course_name: CourseName,
    pub credit: String,
    pub degree: String,
    pub department: String,
    pub faculty: String,
    pub course_status: String,
    pub course_condition: Option<Vec<String>>,
    pub continue_course: Option<Vec<String>>,
    pub equivalent_course: Option<Vec<String>>,
    pub sections_count: usize,
    pub sections: Vec<Section>,
}
