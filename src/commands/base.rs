use std::str::FromStr;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use crate::cli::{is_valid_time, convert_valid_time_to_timezone_utc, display_all_zones, display_selected_zones};

pub fn run(base_tz_raw: &str, time_str: &str, zones_arg: Option<&str>, use_alias_labels: bool) {
    // Validate time (defense-in-depth)
    if !is_valid_time(time_str) {
        eprintln!("invalid time format: '{}'. expected H:MM or HH:MM (00-23:00-59)", time_str);
        return;
    }

    // Resolve base timezone: alias -> canonical IANA -> Tz
    let canonical = crate::config::normalize_zone_name(base_tz_raw)
        .unwrap_or_else(|| base_tz_raw.to_string());
    let tz = match Tz::from_str(&canonical) {
        Ok(tz) => tz,
        Err(_) => {
            eprintln!("unknown base timezone: '{}'. Try IANA name like 'Asia/Tokyo'", base_tz_raw);
            return;
        }
    };

    // Build base UTC time from the provided local time in base tz
    let base_time: DateTime<Utc> = match convert_valid_time_to_timezone_utc(time_str, &tz) {
        Ok(utc) => utc,
        Err(e) => {
            eprintln!("failed to construct base time: {}", e);
            return;
        }
    };

    // 1) CLI --zones overrides
    if let Some(zs) = zones_arg {
        let zones: Vec<String> = zs
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !zones.is_empty() {
            display_selected_zones(&base_time, &zones, use_alias_labels);
            return;
        }
    }

    // 2) Config file
    if let Some(zones) = crate::config::load_zones() {
        if !zones.is_empty() {
            display_selected_zones(&base_time, &zones, use_alias_labels);
            return;
        }
    }

    // 3) Fallback
    display_all_zones(&base_time, use_alias_labels);
}
