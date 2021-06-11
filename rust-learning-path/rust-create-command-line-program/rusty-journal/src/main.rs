use anyhow::anyhow;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

use std::path::PathBuf;

use std::env::current_dir;

fn find_default_journal_file() -> Option<PathBuf> {
    let cur_dir = match current_dir() {
        Ok(path) => path,
        Err(_) => panic!("current dir not found"),
    };

    let mut path = PathBuf::new();
    path.push(cur_dir);
    path.push(".rusty-journal.json");
    Some(path)
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}

#[test]
fn print_dir() {
    println!("{:?}", find_default_journal_file());
}
