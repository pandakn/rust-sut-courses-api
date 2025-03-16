use super::{
    build_search_url::build_search_url, courses_data_scraper::scrape_course_data, fetch::fetch_html,
};
use crate::{
    domain::{course::CourseSearch, course_scraper::CourseScraper},
    presentation::dto::course_response::GroupedCourse,
};
use anyhow::Result;
use axum::async_trait;

pub struct CourseUrlScraper {
    base_url: String,
}

impl CourseUrlScraper {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl CourseScraper for CourseUrlScraper {
    async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<GroupedCourse>> {
        let url = build_search_url(self.base_url.clone(), search);

        let html = fetch_html(&url).await?;

        let grouped_courses = scrape_course_data(&html).await?;

        Ok(grouped_courses)
    }
}
