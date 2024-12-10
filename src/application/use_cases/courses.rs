use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        course::{CourseSearch, CourseTableSearch},
        course_scraper::CourseScraper,
    },
    presentation::dto::course_response::{CourseDetailResponse, CourseTableResponse},
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

    pub async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<CourseDetailResponse>> {
        let courses = self.course_scraper.search_courses(search).await?;

        Ok(courses)
    }

    pub async fn get_courses(
        &self,
        search: &CourseTableSearch,
    ) -> Result<Vec<CourseTableResponse>> {
        let courses = self.course_scraper.get_courses(search).await?;

        Ok(courses)
    }
}
