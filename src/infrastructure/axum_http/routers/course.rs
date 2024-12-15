use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{
    application::use_cases::courses::SearchCoursesUseCase,
    domain::{course::CourseSearch, course_scraper::CourseScraper},
    infrastructure::scraper::course_scraper::CourseUrlScraper,
};

// Define the function to create the router
pub fn routes(base_url: String) -> Router {
    let course_scraper = CourseUrlScraper::new(base_url);
    let courses_use_case = SearchCoursesUseCase::new(Arc::new(course_scraper));

    Router::new()
        .route("/", get(search_courses))
        .with_state(Arc::new(courses_use_case))
}

pub async fn search_courses<T>(
    State(courses_use_case): State<Arc<SearchCoursesUseCase<T>>>,
    search: Query<CourseSearch>,
) -> impl IntoResponse
where
    T: CourseScraper + Send + Sync,
{
    match courses_use_case.search_courses(&search).await {
        Ok(courses) => (StatusCode::OK, Json(courses)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
