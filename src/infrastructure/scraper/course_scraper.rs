use super::{
    build_search_url::{build_search_table_url, build_search_url},
    course_details_scraper::scrape_course_details,
    courses_table_scraper::scrape_courses_table,
    fetch::fetch_html,
};
use crate::{
    domain::{
        course::{CourseSearch, CourseTableSearch},
        course_scraper::CourseScraper,
    },
    presentation::dto::course_response::{CourseDetailResponse, CourseTableResponse},
};
use anyhow::{Ok, Result};
use axum::async_trait;

pub struct CourseParser {
    base_url: String,
}

impl CourseParser {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl CourseScraper for CourseParser {
    async fn search_courses(&self, search: &CourseSearch) -> Result<Vec<CourseDetailResponse>> {
        let url = build_search_url(self.base_url.clone(), search);

        // Fetch HTML content from the constructed URL
        let html = fetch_html(&url).await?;

        // Parse the fetched HTML content
        if let Some(course_details) = scrape_course_details(&html)? {
            println!("{:?}", course_details);
            Ok(vec![course_details])
        } else {
            println!("Course details not found.");
            Ok(vec![]) // Return empty vector if no courses found
        }
    }

    async fn get_courses(&self, search: &CourseTableSearch) -> Result<Vec<CourseTableResponse>> {
        let url = build_search_table_url(self.base_url.clone(), search);

        // Fetch HTML content from the constructed URL
        let html = fetch_html(&url).await?;

        // Parse the fetched HTML content
        let courses_table = scrape_courses_table(&html)?;

        Ok(courses_table)
    }
}
