use crate::presentation::dto::course_response::CourseDetailResponse;
use anyhow::{anyhow, Context, Result};
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, warn};

/// Scrapes course details from HTML, extracting key information
///
/// # Arguments
///
/// * `html` - A string slice containing the HTML to parse
///
/// # Returns
///
/// A `Result` containing an `Option` of `CourseDetailResponse` if parsing is successful
pub fn scrape_course_details(html: &str) -> Result<Option<CourseDetailResponse>> {
    // Early return for empty or whitespace-only HTML
    if html.trim().is_empty() {
        debug!("Received empty HTML for course details scraping");
        return Ok(None);
    }

    // Precompile selectors with manual error handling
    let course_details_selector = Selector::parse(
        "td:nth-child(3) > table:nth-child(2) > tbody > tr > td:nth-child(2) > table > tbody",
    )
    .map_err(|e| anyhow!("Failed to parse course details selector: {}", e))?;

    let course_name_en_selector =
        Selector::parse("tr:nth-child(1) > td:nth-child(2) > b > font")
            .map_err(|e| anyhow!("Failed to parse course name (English) selector: {}", e))?;

    let course_name_th_selector = Selector::parse("tr:nth-child(2) > td:nth-child(2) > font")
        .map_err(|e| anyhow!("Failed to parse course name (Thai) selector: {}", e))?;

    let faculty_selector = Selector::parse("tr:nth-child(3) > td:nth-child(3) > font")
        .map_err(|e| anyhow!("Failed to parse faculty selector: {}", e))?;

    let course_status_selector = Selector::parse("tr:nth-child(5) > td:nth-child(3) > font")
        .map_err(|e| anyhow!("Failed to parse course status selector: {}", e))?;

    // Parse the HTML document
    let document = Html::parse_document(html);

    // Find the course details table
    let course_table = document
        .select(&course_details_selector)
        .next()
        .context("No course details table found in HTML")?;

    // Extract and sanitize course details with logging
    let course_name_en =
        extract_text(&course_table, &course_name_en_selector).unwrap_or_else(|| {
            warn!("Could not extract English course name");
            "N/A".to_string()
        });

    let course_name_th =
        extract_text(&course_table, &course_name_th_selector).unwrap_or_else(|| {
            warn!("Could not extract Thai course name");
            "N/A".to_string()
        });

    let faculty_data = extract_text(&course_table, &faculty_selector).unwrap_or_else(|| {
        warn!("Could not extract faculty information");
        "N/A".to_string()
    });

    let course_status = extract_text(&course_table, &course_status_selector).unwrap_or_else(|| {
        warn!("Could not extract course status");
        "N/A".to_string()
    });

    // Split faculty and department with robust handling
    let (faculty, department) = split_faculty_data(&faculty_data);

    // Construct and return the course detail response
    Ok(Some(CourseDetailResponse {
        course_name_en,
        course_name_th,
        faculty,
        department,
        course_status,
    }))
}

/// Extracts text from a selected element, with optional logging
///
/// # Arguments
///
/// * `table` - The parent element to search within
/// * `selector` - The selector to find the target element
///
/// # Returns
///
/// An `Option` containing the trimmed text, or `None` if not found
fn extract_text<'a>(table: &ElementRef, selector: &Selector) -> Option<String> {
    table
        .select(selector)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
}

/// Splits faculty data into faculty and department
///
/// # Arguments
///
/// * `faculty_data` - The raw faculty information string
///
/// # Returns
///
/// A tuple of (faculty, department)
fn split_faculty_data(faculty_data: &str) -> (String, String) {
    faculty_data
        .split_once(", ")
        .map(|(f, d)| (f.trim().to_string(), d.trim().to_string()))
        .unwrap_or_else(|| (faculty_data.to_string(), "N/A".to_string()))
}
