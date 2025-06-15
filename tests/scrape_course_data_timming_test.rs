mod helpers;

#[cfg(test)]
mod scrape_course_data_timming_test {
    use std::time::Instant;

    use rust_sut_courses_api::infrastructure::scraper::courses_data_scraper::{
        scrape_course_data, scrape_course_data_v2,
    };

    use crate::helpers::load_mock_html;

    #[tokio::test]
    async fn simple_timing_benchmark() {
        let mock_html = load_mock_html("mock_html_course_table_523*.txt");
        let iterations = 10;

        // Benchmark scrape_course_data_v2
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = scrape_course_data_v2(&mock_html).await.unwrap();
        }
        let v2_duration = start.elapsed();

        // Benchmark scrape_course_data
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = scrape_course_data(&mock_html).await.unwrap();
        }
        let v1_duration = start.elapsed();

        // Convert to seconds with precision
        let v2_avg_secs = v2_duration.as_secs_f64() / iterations as f64;
        let v1_avg_secs = v1_duration.as_secs_f64() / iterations as f64;
        let v2_total_secs = v2_duration.as_secs_f64();
        let v1_total_secs = v1_duration.as_secs_f64();

        println!("=== Benchmark Results ===");
        println!("Iterations: {}", iterations);
        println!();
        println!("scrape_course_data_v2:");
        println!("  Total time: {:.4} seconds", v2_total_secs);
        println!("  Average time: {:.4} seconds", v2_avg_secs);
        println!();
        println!("scrape_course_data:");
        println!("  Total time: {:.4} seconds", v1_total_secs);
        println!("  Average time: {:.4} seconds", v1_avg_secs);
        println!();

        let difference_secs = (v2_total_secs - v1_total_secs).abs();

        if v2_total_secs < v1_total_secs {
            let improvement_pct = ((v1_total_secs - v2_total_secs) / v1_total_secs) * 100.0;
            println!(
                "✅ v2 is FASTER by {:.4} seconds ({:.2}% improvement)",
                difference_secs, improvement_pct
            );
        } else {
            let regression_pct = ((v2_total_secs - v1_total_secs) / v1_total_secs) * 100.0;
            println!(
                "❌ v1 is FASTER by {:.4} seconds ({:.2}% better than v2)",
                difference_secs, regression_pct
            );
        }

        // Throughput comparison
        let v2_throughput = iterations as f64 / v2_total_secs;
        let v1_throughput = iterations as f64 / v1_total_secs;

        println!();
        println!("Throughput:");
        println!("  v2: {:.2} operations/second", v2_throughput);
        println!("  v1: {:.2} operations/second", v1_throughput);
    }

    #[tokio::test]
    async fn memory_usage_comparison() {
        let mock_html = load_mock_html("mock_html_course_table_523*.txt");

        // You could use a memory profiler here or just measure execution time
        // with different concurrency levels

        println!("Testing memory usage patterns...");

        // Test with concurrent execution
        let tasks = (0..5).map(|_| {
            let html = mock_html.clone();
            tokio::spawn(async move { scrape_course_data_v2(&html).await.unwrap() })
        });

        let results = futures::future::join_all(tasks).await;
        println!("Concurrent v2 executions completed: {}", results.len());

        // Test with sequential execution
        for i in 0..5 {
            let _ = scrape_course_data(&mock_html).await.unwrap();
            println!("Sequential v1 execution {} completed", i + 1);
        }
    }
}
