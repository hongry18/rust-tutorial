use anyhow::anyhow;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

use std::path::PathBuf;

fn find_default_journal_file() -> Option<PathBuf> {
    let mut path = PathBuf::from(
        r"/Users/hongry/workspace/work/individual/rust-by-example/rust-learning-path/rust-create-command-line-program/rusty-journal",
    );
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