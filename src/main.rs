use clap::{ArgGroup, Parser};
 

mod commands;
mod cli; // expose cli so other modules can use validators
mod config; // configuration loader for zones

/// nanji: a simple CLI that shows the current time in multiple timezones
#[derive(Parser)]
#[command(name = "nanji")]
#[command(about = "Show current times in Japan, US, and other major cities", long_about = None)]
#[command(group(ArgGroup::new("base_tz").args(&["tokyo", "dallas"])))]
pub struct Cli {
    #[arg(short, long)]
    pub dallas: Option<String>,

    #[arg(short, long)]
    pub tokyo: Option<String>,
    /// Comma-separated list of zones (e.g. tokyo,dallas)
    #[arg(short, long)]
    pub zones: Option<String>,

    /// Use alias names for labels (e.g. "tokyo") instead of canonical IANA names
    #[arg(short = 'a', long = "alias")]
    pub alias: bool,
}

fn main() {
    let cli = Cli::parse();

    // Delegate branching to the commands module to keep main tidy
    commands::run(&cli);
}

 
