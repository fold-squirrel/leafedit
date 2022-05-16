use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

mod commadline;
use commadline::{Opr, Cli, Tasks};

mod patch;
mod edit;
mod list;
mod grid;
mod merge;
mod info;

pub const CREATOR: &str = "Ahmed Mohammed (ahmed_alaa_gomaa@outlook.com)";
pub const PRODUCER: &str = "leafedit (https://github.com/navyleaf/leafedit)";

fn main() {
    let cli = Cli::parse();
    start(cli).ok();
}

fn start(args: commadline::Cli) -> Result<(), u32> {
    match args.task {

        Tasks::Patch { file, save_as, page } => {
            patch::patch::patch(&file, &save_as, page).ok();
        },

        Tasks::Edit { operations, opr_file, undo, file, save_as} => {
            if undo {
                edit::undo::undo_last(file, save_as).ok();
            } else if let Some(path) = opr_file {
                let oprs = parse_opr_file(&path);
                edit::apply::edits(&file, &save_as, oprs).ok();
            } else {
                edit::apply::edits(&file, &save_as, operations).ok();
            }
        }

        Tasks::Merge { files, save_as } => {
            merge::merger::merge_patched_docs(files, save_as).ok();
        }

        Tasks::Grid { gridtype} => {
            grid::grids::generate(gridtype).ok();
        }

        Tasks::List { list } => {
            list::help::of(list);
        }

        Tasks::Info { about } => {
            info::display::from_file(about).ok();
        }
    }

    Ok(())
}

fn parse_opr_file(path: &str) -> Vec<Opr> {
    let mut oprs = vec![];
    let lines = read_lines(path);
    for line in lines.flatten() {
        if let Ok(opr) = Opr::from_str(&line) {
            oprs.push(opr)
        }
    }
    oprs
}

fn read_lines(file: &str) -> io::Lines<io::BufReader<File>> {
    if let Ok(file) = File::open(file) {
        io::BufReader::new(file).lines()
    } else {
        std::panic::set_hook(Box::new(|_| {
            println!("operations file not found");
        }));
        panic!();
    }
}
