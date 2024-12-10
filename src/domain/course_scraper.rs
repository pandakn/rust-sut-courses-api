use crate::presentation::dto::course_response::{CourseDetailResponse, CourseTableResponse};

use super::course::{CourseSearch, CourseTableSearch};

use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait CourseScraper {
    async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<CourseDetailResponse>>;
    async fn get_courses(&self, search: &CourseTableSearch) -> Result<Vec<CourseTableResponse>>;
}
