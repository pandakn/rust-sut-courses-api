use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseDetailResponse {
    pub course_name_en: String,
    pub course_name_th: String,
    pub faculty: String,
    pub department: String,
    pub course_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTableResponse {
    pub url: String,
    pub course_code: String,
    pub credit: String,
}
