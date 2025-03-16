// TODO: implement rest data and refactor
use crate::{
    infrastructure::scraper::{extract_exam_info::extract_exam_info, utils},
    presentation::dto::course_response::{
        CourseDetailResponse, CourseRequirements, Exam, ExamInfo,
    },
};
use anyhow::{anyhow, Context, Ok, Result};
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
// TODO: implement logic to scrape course exam info
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

    let course_requirements = scrape_course_requirements(&html)?;
    let course_condition = course_requirements.course_condition;
    let continue_course = course_requirements.continue_course;
    let equivalent_course = course_requirements.equivalent_course;

    let course_exams = scrape_course_exams(&html)?;

    let midterm = course_exams.midterm;
    let final_exam = course_exams.r#final;

    // Construct and return the course detail response
    Ok(Some(CourseDetailResponse {
        course_name_en,
        course_name_th,
        faculty,
        department,
        course_status,
        course_condition,
        continue_course,
        equivalent_course,
        midterm,
        r#final: final_exam,
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

// *Need refactor
fn extract_course_conditions(html: &str, label: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    // Selector to get all rows in the table
    let row_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    let mut data = Vec::new();

    // Iterate over each table row
    for row in document.select(&row_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();
        if tds.len() >= 2 {
            // Check if the second column contains the label
            if let Some(label_text) = tds[1].text().next() {
                if label_text.contains(label) {
                    // Extract course IDs from the third column
                    for link in tds[2].select(&link_selector) {
                        if let Some(course_id) = link.text().next() {
                            data.push(course_id.to_string());
                        }
                    }
                }
            }
        }
    }

    data
}

pub fn scrape_course_requirements(html: &str) -> Result<CourseRequirements> {
    // Get values and remove duplicates using HashSet
    let mut course_condition: Vec<String> = extract_course_conditions(html, "เงื่อนไขรายวิชา");
    course_condition.dedup();

    let mut continue_course: Vec<String> = extract_course_conditions(html, "รายวิชาต่อเนื่อง");
    continue_course.dedup();

    let mut equivalent_course: Vec<String> = extract_course_conditions(html, "รายวิชาเทียบเท่า");
    equivalent_course.dedup();

    Ok(CourseRequirements {
        course_condition,
        continue_course,
        equivalent_course,
    })
}

/// Extracts exam data (e.g., midterm or final) based on the keyword.
fn extract_exam_data(document: &Html, keyword: &str) -> Result<Option<ExamInfo>> {
    let elements = utils::select_contains(document, "td", keyword);

    for element in elements {
        if let Some(sibling) = utils::get_next_sibling(element) {
            let raw_text_exam_info = sibling
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            if let Some(exam_info) = extract_exam_info(&raw_text_exam_info) {
                // println!("Extracted Exam Info: {:?}", exam_info);
                return Ok(Some(exam_info));
            }
        }
    }

    Ok(None)
}

pub fn scrape_course_exams(html: &str) -> Result<Exam> {
    let document = Html::parse_document(html);

    // Extract midterm exam info
    let midterm_exam = extract_exam_data(&document, "สอบกลางภาค")?;

    // Extract final exam info
    let final_exam = extract_exam_data(&document, "สอบประจำภาค")?;

    Ok(Exam {
        midterm: midterm_exam,
        r#final: final_exam,
    })
}
