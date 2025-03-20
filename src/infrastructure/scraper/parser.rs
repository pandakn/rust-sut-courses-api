use crate::{
    infrastructure::scraper::course_details_scraper::scrape_course_details,
    presentation::dto::course_response::{ClassSchedule, CourseBase, CourseDetailResponse, Seat},
    utils::string_utils::{extract_value_in_brackets, trim_space},
};
use scraper::ElementRef;

use super::{constants::COURSE_STATUS_OBJ, fetch::fetch_html, selector::CourseRowSelectors};

pub struct CourseRowParser<'a> {
    row: ElementRef<'a>,
    selectors: &'a CourseRowSelectors,
}

impl<'a> CourseRowParser<'a> {
    pub fn new(row: ElementRef<'a>, selectors: &'a CourseRowSelectors) -> Self {
        Self { row, selectors }
    }

    pub fn parse_base_data(&self) -> CourseBase {
        let (course_code, version) = self.parse_course_code();
        let url = self.parse_course_url();
        let note = self.parse_note();
        let professors = self.parse_professors();
        let credit = self.get_text(&self.selectors.credit);
        let section = self.get_text(&self.selectors.section);
        let status_section = self.parse_status();
        let language = self.parse_language();
        let degree = self.get_text(&self.selectors.degree);
        let class_schedule = self.parse_schedule();
        let seat = self.parse_seat();

        CourseBase {
            course_code,
            version,
            url,
            note,
            professors,
            credit,
            section,
            status_section,
            language,
            degree,
            class_schedule,
            seat,
        }
    }

    pub async fn fetch_course_details(url: &str) -> Option<CourseDetailResponse> {
        let html = fetch_html(url).await.unwrap();
        scrape_course_details(&html).unwrap()
    }

    fn parse_course_code(&self) -> (String, String) {
        let code_str = self.get_text(&self.selectors.course_code);
        self.transform_course_code(&code_str)
    }

    fn parse_course_url(&self) -> String {
        self.row
            .select(&self.selectors.course_link)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| format!("http://reg.sut.ac.th/registrar/{}", href))
            .unwrap_or_default()
            .trim()
            .to_string()
    }

    fn parse_professors(&self) -> Vec<String> {
        self.row
            .select(&self.selectors.professors)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    fn parse_note(&self) -> Option<String> {
        let note_text = self.get_text(&self.selectors.note);
        Some(extract_value_in_brackets(&note_text)?.to_string())
    }

    fn parse_seat(&self) -> Seat {
        Seat {
            total_seat: self.get_text(&self.selectors.seat_total),
            registered: self.get_text(&self.selectors.seat_registered),
            remain: self.get_text(&self.selectors.seat_remain),
        }
    }

    fn parse_schedule(&self) -> Vec<ClassSchedule> {
        let rooms: Vec<String> = self
            .row
            .select(&self.selectors.rooms)
            .map(|el| trim_space(&el.text().collect::<String>()))
            .collect();

        let schedule_str = self.get_text(&self.selectors.schedule);
        let schedules = self.transform_schedule(&schedule_str);
        self.merge_schedules_with_rooms(schedules, rooms)
    }

    fn parse_status(&self) -> String {
        let status_code = self.get_text(&self.selectors.status);
        COURSE_STATUS_OBJ
            .get(status_code.as_str())
            .unwrap_or(&"Unknown")
            .to_string()
    }

    fn parse_language(&self) -> String {
        trim_space(
            self.get_text(&self.selectors.language)
                .split(':')
                .next()
                .unwrap_or(""),
        )
    }

    fn get_text(&self, selector: &scraper::Selector) -> String {
        trim_space(
            &self
                .row
                .select(selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default(),
        )
    }

    fn merge_schedules_with_rooms(
        &self,
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

    fn transform_course_code(&self, course_code_str: &str) -> (String, String) {
        let parts: Vec<&str> = course_code_str.split('-').collect();
        let course_code = parts[0].trim().to_string();
        let version = parts.get(1).unwrap_or(&"").trim().to_string();
        (course_code, version)
    }

    fn transform_schedule(&self, schedule: &str) -> Vec<ClassSchedule> {
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
}
