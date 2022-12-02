use core::simulation::{RunOption, Simulator};

use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("run")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("best")
                .short('b')
                .long("best")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("worst")
                .short('w')
                .long("worst")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .short('F')
                .long("files")
                .action(ArgAction::Append),
        )
        .get_matches();

    let all = matches.get_flag("all");
    let first = matches.get_flag("first");
    let best = matches.get_flag("best");
    let worst = matches.get_flag("worst");

    let files = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let mut simulation = Simulator::new(files, RunOption::from((all, first, best, worst)));
    simulation.run()
}
