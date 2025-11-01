use crate::cli::{convert_valid_time_to_timezone_utc, is_valid_time, display_all_zones, display_selected_zones};

pub fn run(time_str: &str, zones_arg: Option<&str>, use_alias_labels: bool) {
    // validate
    if !is_valid_time(time_str) {
        eprintln!("invalid time format: '{}'. expected H:MM or HH:MM (00-23:00-59)", time_str);
        return;
    }

    // build base datetime at Tokyo for today with given time
    let tz = chrono_tz::Asia::Tokyo;
    let base_time = convert_valid_time_to_timezone_utc(time_str, &tz).unwrap();

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
    display_all_zones(&base_time);
}
