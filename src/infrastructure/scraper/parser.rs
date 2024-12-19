use crate::{
    infrastructure::scraper::course_details_scraper::scrape_course_details,
    presentation::dto::course_response::{
        ClassSchedule, Course, CourseDetailResponse, CourseDetails, Seat,
    },
    utils::string_utils::{extract_value_in_brackets, trim_space},
};
use scraper::ElementRef;
use uuid::Uuid;

use super::{constants::COURSE_STATUS_OBJ, fetch::fetch_html, mock, selector::CourseRowSelectors};

pub struct CourseRowParser<'a> {
    row: ElementRef<'a>,
    selectors: &'a CourseRowSelectors,
}

impl<'a> CourseRowParser<'a> {
    pub fn new(row: ElementRef<'a>, selectors: &'a CourseRowSelectors) -> Self {
        Self { row, selectors }
    }

    pub fn parse(&self) -> Course {
        let (course_code, version) = self.parse_course_code();

        let url = self.parse_course_url();

        let course_result: Course = Course {
            id: Uuid::new_v4().to_string(),
            url,
            course_code,
            version,
            course_name_en: String::from("mock course name en"), // TODO: Implement actual parsing
            course_name_th: Some(String::from("mock course name th")), // TODO: Implement actual parsing
            faculty: String::from("mock faculty"), // TODO: Implement actual parsing
            department: String::from("mock department"), // TODO: Implement actual parsing
            note: self.parse_note(),
            professors: self.parse_professors(),
            credit: self.get_text(&self.selectors.credit),
            section: self.get_text(&self.selectors.section),
            status_section: self.parse_status(),
            language: self.parse_language(),
            degree: self.get_text(&self.selectors.degree),
            class_schedule: self.parse_schedule(),
            seat: self.parse_seat(),
            details: self.parse_course_details(),
        };

        course_result
    }

    // TODO: Implement actual course details parsing
    async fn _get_course_details(&self, url: String) -> Option<CourseDetailResponse> {
        let html = fetch_html(&url).await.unwrap();

        let course_details = scrape_course_details(html.as_str()).unwrap();

        course_details
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
            &self
                .get_text(&self.selectors.language)
                .split(':')
                .next()
                .unwrap_or("")
                .to_string(),
        )
    }

    fn parse_course_details(&self) -> CourseDetails {
        // TODO: Implement actual course details parsing
        mock::mock_course_details()
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
