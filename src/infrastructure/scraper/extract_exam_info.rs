use regex::Regex;

use crate::{presentation::dto::course_response::ExamInfo, utils::string_utils};

use super::constants::MONTH_ABBREVIATIONS;

fn convert_month_abbreviation(month_abbr: &str) -> String {
    MONTH_ABBREVIATIONS
        .get(month_abbr)
        .unwrap_or(&month_abbr)
        .to_string()
}

pub fn extract_exam_info(raw_text: &str) -> Option<ExamInfo> {
    if raw_text.trim().is_empty() {
        return None;
    }

    let parts: Vec<&str> = raw_text.split("เวลา").collect();
    if parts.len() < 2 {
        return None;
    }

    let date_part = parts[0].trim();
    let time_part = parts[1].trim();

    let time_range_pattern = Regex::new(r"^\d{2}:\d{2}\s*-\s*\d{2}:\d{2}").unwrap();
    let time_part_no_whitespace = time_part.replace(" ", "");

    let extracted_time_range = time_range_pattern
        .find(&time_part_no_whitespace)
        .map_or("", |m| m.as_str());

    let extracted_room = time_part_no_whitespace
        .split(extracted_time_range)
        .nth(1)
        .unwrap_or("")
        .trim();

    // let cleaned_time_range = clean_string(extracted_time_range);
    let cleaned_time_range = string_utils::trim_space(extracted_time_range);

    let date_parts: Vec<&str> = date_part.split_whitespace().collect();
    if date_parts.len() < 3 {
        return None;
    }

    let date = date_parts[0].to_string();
    let month = convert_month_abbreviation(date_parts[1]);
    let year = date_parts[2].to_string();

    Some(ExamInfo {
        date,
        month,
        times: cleaned_time_range,
        year,
        room: extracted_room.to_string(),
    })
}
