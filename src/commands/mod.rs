pub mod show;
pub mod base;
use crate::Cli;

pub fn run(cli: &Cli) {
    if let (Some(base), Some(time)) = (cli.base.as_deref(), cli.time.as_deref()) {
        base::run(base, time, cli.zones.as_deref(), cli.alias);
        return;
    }

    show::run(cli.zones.as_deref(), cli.alias);
}
