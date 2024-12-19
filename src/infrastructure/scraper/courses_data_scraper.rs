use anyhow::Result;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

use crate::presentation::dto::course_response::{Course, CourseName, Exam, GroupedCourse, Section};

use super::{parser::CourseRowParser, selector::CourseRowSelectors};

pub fn scrape_course_data(html: &str) -> Result<Vec<GroupedCourse>> {
    // Early return for empty or whitespace-only HTML
    if html.trim().is_empty() {
        debug!("Received empty or whitespace-only HTML for course scraping");
        return Ok(Vec::new());
    }

    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table:nth-child(2) tr[valign]").unwrap();

    let mut courses: Vec<Course> = Vec::new();

    for row in document.select(&row_selector) {
        let course_row_selectors =
            CourseRowSelectors::new().expect("Failed to initialize selectors");
        let course_parser = CourseRowParser::new(row, &course_row_selectors);

        let course = course_parser.parse();

        let data_obj = Course {
            id: Uuid::new_v4().to_string(),
            url: course.url,
            course_code: course.course_code,
            version: course.version,
            course_name_en: course.course_name_en,
            course_name_th: course.course_name_th,
            faculty: course.faculty,
            department: course.department,
            note: course.note,
            professors: course.professors,
            credit: course.credit,
            section: course.section,
            status_section: course.status_section,
            language: course.language,
            degree: course.degree,
            class_schedule: course.class_schedule,
            seat: course.seat,
            details: course.details,
        };

        courses.push(data_obj);
    }

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
