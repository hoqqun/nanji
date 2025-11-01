pub mod show;
pub mod tokyo;
pub mod dallas;
use crate::Cli;

pub fn run(cli: &Cli) {
    if let Some(time) = cli.tokyo.as_deref() {
        tokyo::run(time);
        return;
    }

    if let Some(time) = cli.dallas.as_deref() {
        dallas::run(time);
        return;
    }

    show::run(cli.zones.as_deref());
}
