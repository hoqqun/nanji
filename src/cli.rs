use chrono::{Datelike, TimeZone, DateTime, Utc};
use chrono_tz::Tz;
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


pub fn display_all_zones(base_time: &DateTime<Utc>) {
    // show across zones
    let zones: Vec<(&'static str, Tz)> = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|tz| (tz.name(), tz.clone()))
        .collect();

    let max_name_len = zones.iter().map(|(name, _tz)| name.len()).max().unwrap();

    println!("{}", "────────────────────────────".bright_blue());
    for (name, tz) in zones {
        let local_time = base_time.with_timezone(&tz);

        println!(
            "{:<width$}: {}",
            name.bold(),
            local_time.format("%Y-%m-%d %H:%M"),
            width = max_name_len,
        );
    }
    println!("{}", "────────────────────────────".bright_blue());
}

/// Display only the specified IANA timezone names (e.g., "Asia/Tokyo").
/// Invalid names are skipped with a warning.
pub fn display_selected_zones(base_time: &DateTime<Utc>, zones: &[String], use_alias_labels: bool) {
    use std::str::FromStr;

    let mut items: Vec<(String, Option<chrono_tz::Tz>)> = zones
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

            (label, chrono_tz::Tz::from_str(&canonical).ok())
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
                let local_time = base_time.with_timezone(&tz);
                println!(
                    "{:<width$}: {}",
                    label.bold(),
                    local_time.format("%Y-%m-%d %H:%M"),
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

pub fn convert_valid_time_to_timezone_utc(time_str: &str, tz: &Tz) -> Result<DateTime<Utc>, String> {
   // parse hour and minute
  let mut parts = time_str.split(':');
  let h: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
  let m: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);

  let today_local = Utc::now().with_timezone(tz);
  let (y, mo, d) = (today_local.year(), today_local.month(), today_local.day());

  let base_local = match tz.with_ymd_and_hms(y, mo, d, h, m, 0) {
      chrono::LocalResult::Single(dt) => dt,
      chrono::LocalResult::Ambiguous(dt_early, _dt_late) => dt_early, // pick earliest
      chrono::LocalResult::None => {
          eprintln!("the specified local time does not exist due to DST transition");
          return Err("invalid local time due to DST".into());
      }
  };

  Ok(base_local.with_timezone(&Utc)) // DateTime<Utc>
}