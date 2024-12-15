use tracing::info;

use crate::domain::course::CourseSearch;

pub fn build_search_url(base_url: String, params: &CourseSearch) -> String {
    let faculty_id = "all";

    let url = format!(
        "{}?coursestatus=O00&facultyid={}&acadyear={}&semester={}&coursecode={}&coursename={}&maxrow={}",
        base_url, faculty_id, params.acad_year, params.semester, params.course_code, params.course_name, 50
    );

    info!("Url for scrape: {}", url);

    url
}
