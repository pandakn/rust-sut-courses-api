use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSearch {
    pub acad_year: String,
    pub semester: u8,
    pub course_code: String,
    pub course_name: String,
}
