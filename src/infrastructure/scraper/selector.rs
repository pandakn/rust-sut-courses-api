use scraper::Selector;
use std::error::Error;

pub struct CourseRowSelectors {
    pub course_code: Selector,
    pub course_link: Selector,
    pub professors: Selector,
    pub note: Selector,
    pub credit: Selector,
    pub language: Selector,
    pub degree: Selector,
    pub section: Selector,
    pub status: Selector,
    pub seat_total: Selector,
    pub seat_registered: Selector,
    pub seat_remain: Selector,
    pub rooms: Selector,
    pub schedule: Selector,
}

impl CourseRowSelectors {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            course_code: Selector::parse("td:nth-child(2)")?,
            course_link: Selector::parse("td:nth-child(2) a")?,
            professors: Selector::parse("td:nth-child(3) font[color='#407060'] li")?,
            note: Selector::parse("td:nth-child(3) font[color='#660000']")?,
            credit: Selector::parse("td:nth-child(4)")?,
            language: Selector::parse("td:nth-child(5)")?,
            degree: Selector::parse("td:nth-child(8)")?,
            section: Selector::parse("td:nth-child(8)")?,
            status: Selector::parse("td:nth-child(12)")?,
            seat_total: Selector::parse("td:nth-child(9)")?,
            seat_registered: Selector::parse("td:nth-child(10)")?,
            seat_remain: Selector::parse("td:nth-child(11)")?,
            rooms: Selector::parse("td:nth-child(7) u")?,
            schedule: Selector::parse("td:nth-child(7) > font")?,
        })
    }
}
