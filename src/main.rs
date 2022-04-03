//use std::collections::BTreeMap;
//use lopdf::{Document, Object, Stream, ObjectId, Dictionary, dictionary};
use serde::{Deserialize, Serialize};
use std::fmt;

mod adding_new_content_object;
mod embed_fonts_example;

mod patch;

use patch::patch;

use std::error::Error;
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(subcommand)]
	task: Tasks,
}

#[derive(Subcommand, Debug, Clone)]
enum Tasks {
	// add string to pdf
	Edit {
		/// operations to edit pdf contents, currently supported operations: {n}{n}
		/// add string to document -> 'addstr(x:<int>,y:<int>,f:<int>,t:<string>)'{n}
		/// x -> x coordinate, {n}
		/// y -> y coordinate, {n}
		/// f -> font size, {n}
		/// t -> text to add {n}
		/// {n}
		/// more coming soon!!!
		#[clap(short, required = true)]
		operations: Vec<Operation>,

		/// /path/to/file
		#[clap(default_value = "patched.pdf")]
		input_file_name: String,

		/// /out/file/path
		#[clap(default_value = "out.pdf")]
		output_file_name: String,
	},

	Patch {
		/// /path/to/file
		input_file_name: String,
		/// /out/file/path
		#[clap(default_value = "patched.pdf")]
		output_file_name: String,
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
enum Operation {
	/// addstr -> 'addstr(x:<int>,y:<int>,f:<int>,t:<string>)'
	addstr {
		/// x coordinate
		x: u32,
		/// y coordinate
		y: u32,
		/// font size
		f: u32,
		/// string to write to pdf
		t: String,
	}
}

impl Error for ParseOperastionError {}

impl fmt::Display for ParseOperastionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug)]
struct ParseOperastionError;

impl FromStr for Operation {
	type Err = ParseOperastionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let operation = match ron::from_str(s) {
			Ok(opr) => Ok(opr),
			Err(_) => Err(ParseOperastionError),
		};
		Ok(operation?)
	}

}

fn main() {
//	demos();
//	patch(&"1.pdf".to_owned());
	let cli = Cli::parse();
	match cli.clone().task {
		Tasks::Patch { input_file_name, output_file_name } => 
			patch(&input_file_name, &output_file_name),
		Tasks::Edit { operations, input_file_name, output_file_name } => println!("nn"),
	}
	println!("{:#?}", cli);
}


#[allow(dead_code)]
fn demos() {
	let args: Vec<String> = std::env::args().collect();
	adding_new_content_object::run_demo(&args[1], args[2].clone());
	embed_fonts_example::run_demo();
	return;
}
