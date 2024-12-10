use std::sync::Arc;

use rust_sut_courses_api::{config::config_loader, infrastructure::axum_http::http_serve::start};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // let url = "https://reg2.sut.ac.th/registrar/class_info_2.asp?courseid=1009172&coursecode=523332&acadyear=2567&semester=2";

    // let html = fetch_html(url).await?;
    // if let Some(course_details) = details_scraper(&html)? {
    //     println!("{:?}", course_details);
    //     println!("Course Name EN: {}", course_details.course_name_en);
    //     println!("Course Name TH: {}", course_details.course_name_th);
    //     println!("Faculty: {}", course_details.faculty);
    //     println!("Department: {}", course_details.department);
    //     println!("Course Status: {}", course_details.course_status);
    // } else {
    //     println!("Course details not found.");
    // }

    // Ok(())

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV {}", e);
            std::process::exit(1)
        }
    };

    info!("ENV has been loaded🎉");

    start(Arc::new(dotenvy_env))
        .await
        .expect("Failed to start server ❌")
}
