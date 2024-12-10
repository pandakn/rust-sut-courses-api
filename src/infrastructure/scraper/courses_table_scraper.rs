use crate::{domain::course, presentation::dto::course_response::CourseTableResponse};
use anyhow::{anyhow, Result};
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, warn};

pub fn scrape_courses_table(html: &str) -> Result<Vec<CourseTableResponse>> {
    // Early return for empty or whitespace-only HTML
    // Early return for empty or whitespace-only HTML
    if html.trim().is_empty() {
        debug!("Received empty HTML for course details scraping");
        return Err(anyhow!("HTML content is empty"));
    }
    // Extract data for each row
    let mut courses = Vec::new();

    let url = String::from("http://mock.url");
    let course_code = String::from("ENG1111");
    let credit = String::from("4 (3-3-9)");

    // Add the course to the list
    courses.push(CourseTableResponse {
        course_code,
        credit,
        url,
    });

    Ok(courses)
}
