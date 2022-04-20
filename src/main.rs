use clap::Parser;

mod commadline;
use commadline::{Opr, Cli, Tasks, PageSize};

mod patch;
mod edit;
mod list;


fn main() {
    let cli = Cli::parse();
    start(cli).ok();
}

fn start(args: commadline::Cli) -> Result<(), u32> {
    match args.task {

        Tasks::Patch { file, save_as, page } => {
            patch::patch::patch(&file, &save_as, page).ok();
        },

        Tasks::Edit { operations, file, save_as, page_size } => {
            edit::apply::edits(&file, &save_as, operations, page_size).ok();
        }

        Tasks::List { list } => {
            list::help::of(list);
        }
    }

    Ok(())
}
