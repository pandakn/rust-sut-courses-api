// TODO: Refactor

use anyhow::Result;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

use crate::{
    presentation::dto::course_response::{
        ClassSchedule, Course, CourseDetails, CourseName, Exam, ExamInfo, GroupedCourse, Seat,
        Section,
    },
    utils::string_utils::{extract_value_in_brackets, trim_space},
};

fn mock_course_details() -> CourseDetails {
    CourseDetails {
        course_status: String::from("Active"),
        course_condition: Some(vec![
            String::from("Must have passed Math 101"),
            String::from("Must be in the 2nd year"),
        ]),
        continue_course: Some(vec![String::from("Advanced Math 201")]),
        equivalent_course: Some(vec![String::from("Math 101B"), String::from("Math 101C")]),
        mid_exam: Some(ExamInfo {
            date: String::from("10"),
            month: String::from("June"),
            times: String::from("10:00 AM"),
            year: String::from("2024"),
            room: String::from("Room 101"),
        }),
        final_exam: Some(ExamInfo {
            date: String::from("20"),
            month: String::from("June"),
            times: String::from("02:00 PM"),
            year: String::from("2024"),
            room: String::from("Room 202"),
        }),
    }
}

pub fn parse_course_code(course_code_str: &str) -> (String, String) {
    let parts: Vec<&str> = course_code_str.split('-').collect();
    let course_code = parts[0].trim().to_string();
    let version = parts.get(1).unwrap_or(&"").trim().to_string();
    (course_code, version)
}

pub fn parse_schedule(schedule: &str) -> Vec<ClassSchedule> {
    let time_regex = regex::Regex::new(r"\d{2}:\d{2}-\d{2}:\d{2}").unwrap();
    let day_regex = regex::Regex::new(r"Mo|Tu|We|Th|Fr|Sa|Su").unwrap();

    let times: Vec<String> = time_regex
        .find_iter(schedule)
        .map(|m| m.as_str().to_string())
        .collect();
    let days: Vec<String> = day_regex
        .find_iter(schedule)
        .map(|m| m.as_str().to_string())
        .collect();

    times
        .into_iter()
        .zip(days)
        .map(|(time, day)| ClassSchedule {
            day,
            times: time,
            room: None,
        })
        .collect()
}

pub fn merge_schedules_with_rooms(
    schedules: Vec<ClassSchedule>,
    rooms: Vec<String>,
) -> Vec<ClassSchedule> {
    schedules
        .into_iter()
        .zip(rooms)
        .map(|(mut schedule, room)| {
            schedule.room = Some(room);
            schedule
        })
        .collect()
}

// Status mapping
lazy_static::lazy_static! {
    static ref STATUS_OBJ: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("A", "เพิ่มผ่าน WEB ได้เท่านั้น");
        m.insert("C", "ปิดไม่รับลง");
        m.insert("D", "ถอนผ่าน WEB ได้เท่านั้น");
        m.insert("N", "เปิดลงปกติ ทำการโดยเจ้าหน้าที่เท่านั้น");
        m.insert("W", "เปิดลงปกติ สามารถลงทะเบียนผ่าน WEB ได้");
        m.insert("X", "เปลี่ยนกลุ่มผ่าน WEB ได้เท่านั้น");
        m
    };
}

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
        let course_code_str = trim_space(
            &row.select(&Selector::parse("td:nth-child(2)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let (course_code, version) = parse_course_code(&course_code_str);

        let course_url = trim_space(
            &row.select(&Selector::parse("td:nth-child(2) a").unwrap())
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| format!("http://reg.sut.ac.th/registrar/{}", href))
                .unwrap_or_default(),
        );

        // let course_details = scrape_course_details(&course_url).unwrap_or_default();

        let professors: Vec<String> = row
            .select(&Selector::parse("td:nth-child(3) font[color='#407060'] li").unwrap())
            .map(|el| el.text().collect::<String>())
            .collect();

        let note = trim_space(
            &extract_value_in_brackets(
                &row.select(&Selector::parse("td:nth-child(3) font[color='#660000']").unwrap())
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_default(),
            )
            .unwrap_or_default(),
        );

        let credit = trim_space(
            &row.select(&Selector::parse("td:nth-child(4)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let language = trim_space(
            &row.select(&Selector::parse("td:nth-child(5)").unwrap())
                .next()
                .map(|el| {
                    el.text()
                        .collect::<String>()
                        .split(':')
                        .next()
                        .unwrap_or("")
                        .to_string()
                })
                .unwrap_or_default(),
        );

        let degree = trim_space(
            &row.select(&Selector::parse("td:nth-child(8)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let section = trim_space(
            &row.select(&Selector::parse("td:nth-child(8)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let status_section_code = trim_space(
            &row.select(&Selector::parse("td:nth-child(12)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let status_section = STATUS_OBJ
            .get(status_section_code.as_str())
            .unwrap_or(&"Unknown")
            .to_string();

        let total_seat = trim_space(
            &row.select(&Selector::parse("td:nth-child(9)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );
        let registered = trim_space(
            &row.select(&Selector::parse("td:nth-child(10)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );
        let remain = trim_space(
            &row.select(&Selector::parse("td:nth-child(11)").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let seat = Seat {
            total_seat,
            registered,
            remain,
        };

        let rooms: Vec<String> = row
            .select(&Selector::parse("td:nth-child(7) u").unwrap())
            .map(|el| trim_space(&el.text().collect::<String>().trim().to_string()))
            .collect();

        let schedule_str = trim_space(
            &row.select(&Selector::parse("td:nth-child(7) > font").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        );

        let schedules = parse_schedule(&schedule_str);
        let class_schedule = merge_schedules_with_rooms(schedules, rooms);

        let course_details = mock_course_details();

        let details = CourseDetails {
            course_status: course_details.course_status,
            course_condition: course_details.course_condition,
            continue_course: course_details.continue_course,
            equivalent_course: course_details.equivalent_course,
            mid_exam: course_details.mid_exam,
            final_exam: course_details.final_exam,
        };

        let data_obj = Course {
            id: Uuid::new_v4().to_string(),
            url: course_url,
            course_code,
            version,
            course_name_en: String::from("mock course name en"),
            course_name_th: Some(String::from("mock course name th")),
            faculty: String::from("mock faculty"),
            department: String::from("mock department"),
            note: Some(note.to_string()),
            professors,
            credit,
            section,
            status_section,
            language,
            degree,
            class_schedule,
            seat,
            details,
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
