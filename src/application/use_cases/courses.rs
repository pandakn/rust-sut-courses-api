use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{course::CourseSearch, course_scraper::CourseScraper},
    presentation::dto::course_response::GroupedCourse,
};

pub struct SearchCoursesUseCase<T>
where
    T: CourseScraper + Send + Sync,
{
    course_scraper: Arc<T>,
}

impl<T> SearchCoursesUseCase<T>
where
    T: CourseScraper + Send + Sync,
{
    pub fn new(course_scraper: Arc<T>) -> Self {
        Self { course_scraper }
    }

    pub async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<GroupedCourse>> {
        let courses = self.course_scraper.search_courses(search).await?;

        Ok(courses)
    }
}
