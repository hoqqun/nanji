use clap::Parser;
use chrono::{DateTime, Utc};
use colored::*;

/// nanji: a simple CLI that shows the current time in multiple timezones
#[derive(Parser)]
#[command(name = "nanji")]
#[command(about = "Show current times in Japan, US, and other major cities", long_about = None)]
struct Cli {
    /// Comma-separated list of zones (e.g. tokyo,dallas)
    #[arg(short, long)]
    zones: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let now_utc: DateTime<Utc> = Utc::now();

    let default_zones = vec![
        ("Tokyo JST", chrono_tz::Asia::Tokyo),
        ("California PST", chrono_tz::America::Los_Angeles),
        ("Dallas CST", chrono_tz::America::Chicago),
        ("New York EST", chrono_tz::America::New_York),
    ];

    let zones_to_show = if let Some(zones_arg) = cli.zones {
        let lowercase_zones = zones_arg.to_lowercase();
        let filter: Vec<_> = lowercase_zones.split(',').collect();
        default_zones
            .into_iter()
            .filter(|(name, _)| {
                filter.iter().any(|f| name.to_lowercase().contains(f))
            })
            .collect::<Vec<_>>()
    } else {
        default_zones
    };

    println!("{}", "────────────────────────────".bright_blue());
    for (name, tz) in zones_to_show {
        let local_time = now_utc.with_timezone(&tz);
        println!(
            "{}: {}",
            format!("{:<15}", name.bold()),
            local_time.format("%Y-%m-%d %H:%M")
        );
    }
    println!("{}", "────────────────────────────".bright_blue());
}
