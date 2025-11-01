use crate::cli::convert_valid_time_to_timezone_utc;
use crate::cli::is_valid_time;
use crate::cli::display_all_zones;

pub fn run(time_str: &str) {
    // validate
    if !is_valid_time(time_str) {
        eprintln!("invalid time format: '{}'. expected H:MM or HH:MM (00-23:00-59)", time_str);
        return;
    }

    let tz = chrono_tz::America::Chicago;
    let base_time = convert_valid_time_to_timezone_utc(time_str, &tz);

    display_all_zones(&base_time.unwrap())
}
