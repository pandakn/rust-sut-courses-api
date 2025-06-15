use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_sut_courses_api::infrastructure::scraper::courses_data_scraper::{
    scrape_course_data, scrape_course_data_v2,
};
use tokio::runtime::Runtime;

fn load_mock_html(file_name: &str) -> String {
    fs::read_to_string(format!("tests/mocks/html/{}", file_name))
        .expect(&format!("Failed to read mock HTML file {}", file_name))
}

fn bench_versions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mock_html = load_mock_html("mock_html_course_table_523*.txt");

    // Pre-check correctness
    rt.block_on(async {
        let v1 = scrape_course_data(&mock_html).await.unwrap();
        let v2 = scrape_course_data_v2(&mock_html).await.unwrap();

        assert_eq!(v1.len(), v2.len(), "v1 and v2 outputs do not match!");
    });

    c.bench_function("scrape course data v1", |b| {
        b.to_async(&rt)
            .iter(|| async { scrape_course_data(black_box(&mock_html)).await })
    });

    c.bench_function("scrape course data v2", |b| {
        b.to_async(&rt)
            .iter(|| async { scrape_course_data_v2(black_box(&mock_html)).await })
    });
}

criterion_group!(benches, bench_versions);
criterion_main!(benches);
