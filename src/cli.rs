use jiff::{Timestamp, Zoned, tz::TimeZone, civil};
use clap::{Parser, ValueEnum};
use regex::Regex;
use colored::*;


/// Time newtype validated at parse time
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Time(pub String);

/// Validate HH:MM where:
/// - hours: 0-23, allows "9:00" and "09:00" and "00:00"
/// - minutes: 00-59
pub fn is_valid_time(input: &str) -> bool {
    // Accept single-digit hour (0-9) or two-digit 00-23
    // ^(?:[0-9]|0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$
    let re = Regex::new(r"^(?:[0-9]|0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    re.is_match(input)
}

pub fn parse_time(s: &str) -> Result<Time, String> {
    if is_valid_time(s) {
        Ok(Time(s.to_string()))
    } else {
        Err(format!("invalid time format: '{s}'. expected H:MM or HH:MM (00-23:00-59)"))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Zone {
    /// Tokyo (JST)
    Tokyo,
    /// California (PST/PDT)
    California,
    /// Dallas (CST/CDT)
    Dallas,
    /// New York (EST/EDT)
    NewYork,
}


#[derive(Parser, Debug)]
#[command(name = "nanji")]
#[command(about = "Show current times in Japan, US, and other major cities", long_about = None)]
pub struct Cli {
    /// Optional time argument in H:MM or HH:MM (24h)
    #[arg(value_parser = parse_time)]
    pub time: Option<Time>,

    /// Comma-separated list of zones (e.g. tokyo,dallas)
    #[arg(short, long, value_enum, value_delimiter = ',')]
    pub zones: Option<Vec<Zone>>,
}


pub fn display_all_zones(base_time: &Timestamp, use_alias_labels: bool) {
    // Get all available timezone names from jiff-tzdb
    let zones: Vec<(String, TimeZone)> = jiff_tzdb::available()
        .filter_map(|name| {
            TimeZone::get(name).ok().map(|tz| (name.to_string(), tz))
        })
        .collect();

    // Compute labels (alias or canonical)
    let labeled: Vec<(String, TimeZone)> = if use_alias_labels {
        zones
            .into_iter()
            .map(|(canonical, tz)| {
                let label = crate::config::alias_for_canonical(&canonical).unwrap_or(canonical);
                (label, tz)
            })
            .collect()
    } else {
        zones
    };

    let max_name_len = labeled
        .iter()
        .map(|(name, _tz)| name.len())
        .max()
        .unwrap_or(0);

    println!("{}", "────────────────────────────".bright_blue());
    for (label, tz) in labeled {
        let local_time = base_time.to_zoned(tz);
        println!(
            "{:<width$}: {}",
            label.bold(),
            local_time.strftime("%Y-%m-%d %H:%M"),
            width = max_name_len,
        );
    }
    println!("{}", "────────────────────────────".bright_blue());
}

/// Display only the specified IANA timezone names (e.g., "Asia/Tokyo").
/// Invalid names are skipped with a warning.
pub fn display_selected_zones(base_time: &Timestamp, zones: &[String], use_alias_labels: bool) {
    let mut items: Vec<(String, Option<TimeZone>)> = zones
        .iter()
        .map(|raw| {
            let canonical = crate::config::normalize_zone_name(raw)
                .unwrap_or_else(|| raw.clone());
            // choose label
            let label = if use_alias_labels {
                if crate::config::normalize_zone_name(raw).is_some() {
                    raw.clone()
                } else if let Some(alias) = crate::config::alias_for_canonical(&canonical) {
                    alias
                } else {
                    raw.clone()
                }
            } else {
                raw.clone()
            };

            (label, TimeZone::get(&canonical).ok())
        })
        .collect();

    // Compute width using the longest provided label (regardless of validity)
    let max_name_len = items
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);

    println!("{}", "────────────────────────────".bright_blue());
    for (label, tz_opt) in items.drain(..) {
        match tz_opt {
            Some(tz) => {
                let local_time = base_time.to_zoned(tz);
                println!(
                    "{:<width$}: {}",
                    label.bold(),
                    local_time.strftime("%Y-%m-%d %H:%M"),
                    width = max_name_len,
                );
            }
            None => {
                eprintln!("warning: unknown timezone name '{}', skipping", label);
            }
        }
    }
    println!("{}", "────────────────────────────".bright_blue());
}

pub fn convert_valid_time_to_timezone_utc(time_str: &str, tz: &TimeZone) -> Result<Timestamp, String> {
    // parse hour and minute
    let mut parts = time_str.split(':');
    let h: i8 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let m: i8 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Get today's date in the target timezone
    let now = Zoned::now().with_time_zone(tz.clone());
    let today = now.date();

    // Build civil time
    let time = civil::Time::new(h, m, 0, 0)
        .map_err(|e| format!("invalid time: {}", e))?;

    // Build civil datetime
    let dt = civil::DateTime::from_parts(today, time);

    // Convert to zoned datetime in the target timezone
    match dt.to_zoned(tz.clone()) {
        Ok(zdt) => Ok(zdt.timestamp()),
        Err(_) => {
            eprintln!("the specified local time does not exist due to DST transition");
            Err("invalid local time due to DST".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_times_ok() {
        for t in ["0:00", "9:00", "09:00", "09:10", "23:59", "00:00"] {
            assert!(is_valid_time(t), "expected valid: {t}");
        }
    }

    #[test]
    fn invalid_times_ng() {
        for t in ["24:00", "31:00", "09:60", "9:7", "-1:00", "aa:bb", "9:000", "9::00"] {
            assert!(!is_valid_time(t), "expected invalid: {t}");
        }
    }

    #[test]
    fn convert_roundtrip_hour_minute_match_tokyo() {
        let tz = TimeZone::get("Asia/Tokyo").unwrap();
        let ts = convert_valid_time_to_timezone_utc("09:10", &tz).unwrap();
        let local = ts.to_zoned(tz);
        assert_eq!(local.hour(), 9);
        assert_eq!(local.minute(), 10);
    }

    #[test]
    fn convert_roundtrip_hour_minute_match_dallas() {
        let tz = TimeZone::get("America/Chicago").unwrap();
        let ts = convert_valid_time_to_timezone_utc("9:00", &tz).unwrap();
        let local = ts.to_zoned(tz);
        assert_eq!(local.hour(), 9);
        assert_eq!(local.minute(), 0);
    }
}
