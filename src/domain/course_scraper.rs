use crate::presentation::dto::course_response::GroupedCourse;

use super::course::CourseSearch;

use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait CourseScraper {
    async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<GroupedCourse>>;
}
