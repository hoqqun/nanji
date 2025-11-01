use chrono::{DateTime, Utc};
use crate::cli::display_all_zones;

pub fn run(zones_arg: Option<&str>) {
    let now_utc: DateTime<Utc> = Utc::now();

    display_all_zones(&now_utc);
}
