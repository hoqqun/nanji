use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// IANA timezone names, e.g., "Asia/Tokyo", "America/Chicago"
    pub zones: Option<Vec<String>>,
    /// Alias mapping (case-insensitive keys recommended)
    pub aliases: Option<HashMap<String, String>>,
}

/// Load zones list from config file if present.
/// Looks for: $XDG_CONFIG_HOME/nanji/config.toml or ~/.config/nanji/config.toml
pub fn load_zones() -> Option<Vec<String>> {
    let path = find_config_path()?;
    let content = fs::read_to_string(path).ok()?;
    let cfg: Config = toml::from_str(&content).ok()?;
    cfg.zones
}

/// Load full config if present.
pub fn load_config() -> Option<Config> {
    let path = find_config_path()?;
    let content = fs::read_to_string(path).ok()?;
    toml::from_str::<Config>(&content).ok()
}

/// Default built-in aliases.
fn default_aliases() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("tokyo".into(), "Asia/Tokyo".into());
    m.insert("dallas".into(), "America/Chicago".into());
    m.insert("california".into(), "America/Los_Angeles".into());
    m.insert("losangeles".into(), "America/Los_Angeles".into());
    m.insert("los_angeles".into(), "America/Los_Angeles".into());
    m.insert("la".into(), "America/Los_Angeles".into());
    m.insert("newyork".into(), "America/New_York".into());
    m.insert("new_york".into(), "America/New_York".into());
    m.insert("ny".into(), "America/New_York".into());
    m
}

/// Merge built-in aliases with config overrides (config wins).
pub fn alias_map() -> HashMap<String, String> {
    let mut map = default_aliases();
    if let Some(cfg) = load_config() {
        if let Some(mut user) = cfg.aliases {
            // normalize keys to lowercase for case-insensitive lookup
            let mut lower: HashMap<String, String> = HashMap::new();
            for (k, v) in user.drain() {
                lower.insert(k.to_ascii_lowercase(), v);
            }
            map.extend(lower); // user config overrides defaults
        }
    }
    map
}

/// If `raw` matches an alias key (case-insensitive), return the mapped IANA name.
/// Otherwise, return None.
pub fn normalize_zone_name(raw: &str) -> Option<String> {
    let key = raw.to_ascii_lowercase();
    alias_map().remove(&key)
}

/// Given a canonical IANA name, return one of its alias keys if defined.
/// If multiple aliases point to the same canonical name, the lexicographically
/// smallest key is returned for stability.
pub fn alias_for_canonical(canonical: &str) -> Option<String> {
    let mut candidates: Vec<String> = alias_map()
        .into_iter()
        .filter_map(|(k, v)| if v == canonical { Some(k) } else { None })
        .collect();
    if candidates.is_empty() {
        None
    } else {
        candidates.sort_unstable();
        candidates.into_iter().next()
    }
}

fn find_config_path() -> Option<PathBuf> {
    let mut base = dirs::config_dir()?; // ~/.config or platform equivalent
    base.push("nanji");
    base.push("config.toml");
    if base.exists() {
        Some(base)
    } else {
        None
    }
}
