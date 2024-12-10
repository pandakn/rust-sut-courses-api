use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSearch {
    pub acad_year: String,
    pub semester: u8,
    // ! remove course_id, just for debugging
    pub course_id: String,
    pub course_code: String,
    pub course_name: String,
    // pub day: Option<String>,
    // pub time_from: Option<String>,
    // pub time_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTableSearch {
    pub acad_year: String,
    pub semester: u8,
    pub course_code: String,
    pub course_name: String,
}
