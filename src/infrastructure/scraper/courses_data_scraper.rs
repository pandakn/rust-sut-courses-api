use anyhow::Result;
use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use tokio::task::spawn_blocking;
use tracing::debug;
use uuid::Uuid;

use crate::presentation::dto::course_response::{
    Course, CourseBase, CourseDetails, CourseName, Exam, GroupedCourse, Section,
};

use super::{parser::CourseRowParser, selector::CourseRowSelectors};

pub async fn scrape_course_data(html: &str) -> Result<Vec<GroupedCourse>> {
    if html.trim().is_empty() {
        debug!("Received empty or whitespace-only HTML for course scraping");
        return Ok(Vec::new());
    }

    let html_owned = html.to_string();
    let base_data_list = spawn_blocking(move || {
        let document = Html::parse_document(&html_owned);
        let row_selector = Selector::parse("table:nth-child(2) tr[valign]").unwrap();
        let selectors = CourseRowSelectors::new().expect("Failed to initialize selectors");

        document
            .select(&row_selector)
            .map(|row| CourseRowParser::new(row, &selectors).parse_base_data())
            .collect::<Vec<_>>()
    })
    .await?;

    let mut course_futures = Vec::new();

    println!("SSSSSSS: {:#?}", base_data_list);

    for base_data in base_data_list {
        course_futures.push(async move {
            let url = base_data.url.clone();
            let course_details = CourseRowParser::fetch_course_details(&url).await;

            let details = CourseDetails {
                course_status: course_details
                    .as_ref()
                    .map(|d| d.course_status.clone())
                    .unwrap_or_default(),
                course_condition: course_details.as_ref().map(|d| d.course_condition.clone()),
                continue_course: course_details.as_ref().map(|d| d.continue_course.clone()),
                equivalent_course: course_details.as_ref().map(|d| d.equivalent_course.clone()),
                mid_exam: course_details.as_ref().and_then(|d| d.midterm.clone()),
                final_exam: course_details.as_ref().and_then(|d| d.r#final.clone()),
            };

            Course {
                id: Uuid::new_v4().to_string(),
                url,
                course_code: base_data.course_code,
                version: base_data.version,
                course_name_en: course_details
                    .as_ref()
                    .map(|d| d.course_name_en.clone())
                    .unwrap_or_default(),
                course_name_th: course_details
                    .as_ref()
                    .map(|details| details.course_name_th.clone()),
                faculty: course_details
                    .as_ref()
                    .map(|d| d.faculty.clone())
                    .unwrap_or_default(),
                department: course_details
                    .as_ref()
                    .map(|d| d.department.clone())
                    .unwrap_or_default(),
                note: base_data.note,
                professors: base_data.professors,
                credit: base_data.credit,
                section: base_data.section,
                status_section: base_data.status_section,
                language: base_data.language,
                degree: base_data.degree,
                class_schedule: base_data.class_schedule,
                seat: base_data.seat,
                details,
            }
        });
    }

    let courses = futures::future::join_all(course_futures).await;

    Ok(grouped_courses(&courses))
}

fn grouped_courses(courses: &[Course]) -> Vec<GroupedCourse> {
    let mut grouped_map: HashMap<String, GroupedCourse> = HashMap::new();

    for course in courses {
        let key = format!("{}-{}", course.course_code, course.version);

        grouped_map
            .entry(key.clone())
            .or_insert_with(|| GroupedCourse {
                course_code: course.course_code.clone(),
                version: course.version.clone(),
                course_name: CourseName {
                    en: course.course_name_en.clone(),
                    th: course.course_name_th.clone(),
                },
                credit: course.credit.clone(),
                degree: course.degree.clone(),
                department: course.department.clone(),
                faculty: course.faculty.clone(),
                course_status: course.details.course_status.clone(),
                course_condition: course.details.course_condition.clone(),
                continue_course: course.details.continue_course.clone(),
                equivalent_course: course.details.equivalent_course.clone(),
                sections: Vec::new(),
                sections_count: 0,
            });

        if let Some(group) = grouped_map.get_mut(&key) {
            if !group.sections.iter().any(|s| s.section == course.section) {
                let section = Section {
                    id: course.id.clone(),
                    url: course.url.clone(),
                    section: course.section.clone(),
                    status: course.status_section.clone(),
                    note: course.note.clone(),
                    professors: course.professors.clone(),
                    language: course.language.clone(),
                    seat: course.seat.clone(),
                    class_schedule: course.class_schedule.clone(),
                    exams: Exam {
                        midterm: course.details.mid_exam.clone(),
                        r#final: course.details.final_exam.clone(),
                    },
                };
                group.sections.push(section);
            }
            group.sections_count = group.sections.len();
        }
    }

    grouped_map.into_values().collect()
}

pub fn extract_course_base_data(html: &str) -> HashMap<String, CourseBase> {
    let document = Html::parse_document(html);
    let row_selector =
        Selector::parse("table:nth-child(2) tr[valign]").expect("Failed to parse row selector");
    let selectors = CourseRowSelectors::new().expect("Failed to initialize selectors");

    let course_base_list = document
        .select(&row_selector)
        .map(|row| {
            let course_base = CourseRowParser::new(row, &selectors).parse_base_data();
            let format_key = format!("{}-{}", course_base.course_code, course_base.section);
            (format_key, course_base)
        })
        .collect::<HashMap<_, _>>();

    println!("Course Base List: {:#?}", course_base_list);

    return course_base_list;
}

pub fn extract_all_course_urls(html: &str) -> HashMap<String, String> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table:nth-child(2) tr[valign]").unwrap();
    let selectors = CourseRowSelectors::new().expect("Failed to initialize selectors");

    let mut course_urls_map: HashMap<String, String> = HashMap::new();
    let mut seen_course_codes: HashSet<String> = HashSet::new();

    for row in document.select(&row_selector) {
        let url = CourseRowParser::new(row, &selectors).parse_course_url();
        let parsed_url = match Url::parse(&url) {
            Ok(u) => u,
            Err(_) => continue, // Skip malformed URLs
        };

        let course_code = parsed_url
            .query_pairs()
            .find(|(key, _)| key == "coursecode")
            .map(|(_, value)| value.to_string())
            .unwrap_or_default();

        if seen_course_codes.contains(&course_code) {
            continue;
        }

        seen_course_codes.insert(course_code.clone());
        course_urls_map.insert(course_code, url);
    }

    course_urls_map
}

pub async fn fetch_course_details_map(html: &str) -> HashMap<String, CourseDetails> {
    let course_urls_map = extract_all_course_urls(html);

    println!("\n\n Course Urls Length {:?}\n\n", course_urls_map.len());

    let mut course_futures = Vec::new();

    for (course_code, course_url) in course_urls_map {
        let code = course_code.clone();
        let url = course_url.clone();

        course_futures.push(async move {
            let course_details = CourseRowParser::fetch_course_details(&url).await;

            let details = CourseDetails {
                course_status: course_details
                    .as_ref()
                    .map(|d| d.course_status.clone())
                    .unwrap_or_default(),
                course_condition: course_details.as_ref().map(|d| d.course_condition.clone()),
                continue_course: course_details.as_ref().map(|d| d.continue_course.clone()),
                equivalent_course: course_details.as_ref().map(|d| d.equivalent_course.clone()),
                mid_exam: course_details.as_ref().and_then(|d| d.midterm.clone()),
                final_exam: course_details.as_ref().and_then(|d| d.r#final.clone()),
            };

            (code, details)
        });
    }

    let courses = futures::future::join_all(course_futures).await;

    courses.into_iter().collect()
}
