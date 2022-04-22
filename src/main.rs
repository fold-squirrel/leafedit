use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

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

        Tasks::Edit { operations, opr_file, file, save_as, page_size } => {
            if let Some(path) = opr_file {
                let oprs = parse_opr_file(&path);
                edit::apply::edits(&file, &save_as, oprs, page_size).ok();
            } else {
                edit::apply::edits(&file, &save_as, operations, page_size).ok();
            }
        }

        Tasks::List { list } => {
            list::help::of(list);
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
