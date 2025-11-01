use chrono::{DateTime, Utc};
use crate::cli::{display_all_zones, display_selected_zones};

/// Show times for zones determined by CLI or config.
/// Priority:
/// 1) --zones CLI (comma-separated)
/// 2) Config file zones (~/.config/nanji/config.toml)
/// 3) All zones (fallback)
pub fn run(zones_arg: Option<&str>) {
    let now_utc: DateTime<Utc> = Utc::now();

    // 1) CLI --zones takes precedence if provided
    if let Some(zs) = zones_arg {
        let zones: Vec<String> = zs
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !zones.is_empty() {
            display_selected_zones(&now_utc, &zones);
            return;
        }
    }

    // 2) Try config file
    if let Some(zones) = crate::config::load_zones() {
        if !zones.is_empty() {
            display_selected_zones(&now_utc, &zones);
            return;
        }
    }

    // 3) Fallback to all zones
    display_all_zones(&now_utc);
}
