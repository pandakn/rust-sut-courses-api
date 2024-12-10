use tracing::info;

use crate::domain::course::{CourseSearch, CourseTableSearch};

pub fn build_search_url(base_url: String, params: &CourseSearch) -> String {
    // Example URL format: https://reg2.sut.ac.th/registrar/class_info_2.asp?courseid=1009172&coursecode=523332&acadyear=2567&semester=2
    let url = format!(
        "{}?courseid={}&acadyear={}&semester={}&coursecode={}",
        base_url, params.course_id, params.acad_year, params.semester, params.course_code
    );

    info!("Url for scrape: {}", url);

    url
}
pub fn build_search_table_url(base_url: String, params: &CourseTableSearch) -> String {
    // Example URL format: https://reg2.sut.ac.th/registrar/class_info_2.asp?courseid=1009172&coursecode=523332&acadyear=2567&semester=2
    let url = format!(
        "{}?acadyear={}&semester={}&coursecode={}",
        base_url, params.acad_year, params.semester, params.course_code
    );

    info!("Url for scrape: {}", url);

    url
}
