mod cli;
mod fs;
mod tasks;

use cli::{CommandLineArgs, Action::*};
use fs::{add_task};
use structopt::StructOpt;
use tasks::Task;
use std::path::PathBuf;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.yaml");
        path
    })
}

fn main()
{
    let CommandLineArgs {
        action,
        journal_file
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Error");
    
    match action {
        Add { text } => { add_task(journal_file, Task::new(text)).unwrap(); },
        Done { position: _ } => {},
        List => {}
    }
}
