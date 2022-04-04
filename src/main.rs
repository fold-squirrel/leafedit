//use std::collections::BTreeMap;
//use lopdf::{Document, Object, Stream, ObjectId, Dictionary, dictionary};
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream, StringFormat, dictionary};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;

mod adding_new_content_object;
mod embed_fonts_example;

mod patch;

use patch::patch;

use std::error::Error;
use std::str::FromStr;

use clap::{Parser, Subcommand};

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
		.collect()
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(subcommand)]
	task: Tasks,
}

#[derive(Subcommand, Debug, Clone)]
enum Tasks {
	/// edit pdfs, pdf file must be patched first, using a non-patched pdf will result in
	/// unexpected behavior
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
		operations: Vec<DocOperation>,

		/// /path/to/file
		#[clap(default_value = "patched.pdf")]
		input_file_name: String,

		/// /out/file/path
		#[clap(default_value = "out.pdf")]
		output_file_name: String,
	},

	/// apply a set of operations that are necessary before any edits,
	/// like embeding fonts and reseting content tranformation matrix
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
enum DocOperation {
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

#[derive(Debug)]
enum Opr {
	AddStr {px: u32, py: u32, font_size: u32, pdf_str: String, }
}

impl Error for ParseOperastionError {}

impl fmt::Display for ParseOperastionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug)]
struct ParseOperastionError;

impl FromStr for DocOperation {
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
		Tasks::Edit { operations, input_file_name, output_file_name } => {
			let mut opr_vec: Vec<Opr> = vec![];

			for operation in operations {
				match operation {
					DocOperation::addstr { x, y, f, t } => {
						opr_vec.push(Opr::AddStr { px: x, py: y, font_size: f, pdf_str: edit(t) });
					},
				}
			}

			edit_pdf(input_file_name, output_file_name, opr_vec);
		}
	}
//	println!("{:#?}", cli);
}

fn edit_pdf(file_name: String, out_file: String, opr_vec: Vec<Opr>) {
	println!("{}, {}, {:#?}", file_name, out_file, opr_vec);
	let mut doc = Document::load(file_name).unwrap();
	doc.decompress();
	let first_page = match doc.page_iter().next() {
		Some(id) => id,
		None => panic!("non"),
	};

	let mut content_ids = doc.get_page_contents(first_page);
	let last_contint_id = match content_ids.pop() {
		Some(id) => id,
		None => panic!("wow"),
	};

	let mut oprs: Vec<Operation> = vec![];
	for doc_opr in opr_vec {
		oprs.push(Operation::new("BT", vec![]));
		let (x, y, fs, s) = match doc_opr {
			Opr::AddStr { px, py, font_size: fs, pdf_str: s } => (px, py, fs, s),
		};
		oprs.push(Operation::new("Td", vec![x.into(), y.into()]));
		for font_hex in s.split('|') {
			let mut font_str_iter = font_hex.split(',');
			let font = match font_str_iter.next() {
				Some(font) => font.to_owned(),
				None => panic!("hmm"),
			};
			let text = match font_str_iter.next() {
				Some(font) => font.to_owned(),
				None => panic!("hmm"),
			};
			let hex_vec = match decode_hex(&text) {
				Ok(v) => v,
				Err(_) => panic!("vom"),
			};
			oprs.push(Operation::new("Tf", vec![font.into(), fs.into()]));
			oprs.push(
				Operation::new("Tj", vec![Object::String(hex_vec, StringFormat::Hexadecimal)])
			)
		}
		oprs.push(Operation::new("ET", vec![]));
	}

	let content = Content {
		operations: oprs,
	};

	let obj = Object::Stream(Stream::new(dictionary! {}, content.encode().unwrap()));
	println!("mdd: {:#?}", obj);
	doc.objects.insert(last_contint_id, obj);
	doc.save(out_file).unwrap();
}

enum Font {
	F1,
	F2,
	No,
}

fn edit(text: String) -> String {
	let mut is_escaped = false;
	let mut font_str = "".to_owned();
	let mut symbol = "".to_owned();
	let mut font = Font::No;

	for ch in text.chars() {
		if is_escaped {
			match ch {
				'/' => {
					is_escaped = false;
					match symbol.as_str() {
						"" => {
							match font {
								Font::F1 => font_str.push_str("40"),
								_ =>{
									font_str.push_str("|F1,40");
									font = Font::F1;
								}
							}
						},
						"Correct" => {
							match font {
								Font::F2 => font_str.push_str("01"),
								_ => {
									font_str.push_str("|F2,01");
									font = Font::F2;
								}
							}
						},
						_ => println!("no such symbol: {}, ignoring symbol", symbol),
					}
					symbol = "".to_owned();
				}
				',' => {
					match symbol.as_str() {
						"Correct" => {
							match font {
								Font::F2 => {
									font_str.push_str("01");
								}
								_ => {
										font_str.push_str("|F2,01");
										font = Font::F2;
								}
							}
							symbol = "".to_owned();
						}
						_ => println!("no such symbol: {}, ignoring symbol", symbol),
					}
				}
				_ => symbol.push(ch)
			}
		} else {
			match ch {
				'/' => is_escaped = true,
				'A' => match font {
						Font::F1 => {
							font_str.push_str("01");
						}
						_ => {
							font_str.push_str("|F1,01");
							font = Font::F1;
						}
					}
				'B' => match font {
						Font::F1 => {
							font_str.push_str("02");
						}
						_ => {
							font_str.push_str("|F1,02");
							font = Font::F1;
						}
					}
				'C' => match font {
						Font::F1 => {
							font_str.push_str("03");
						}
						_ => {
							font_str.push_str("|F1,03");
							font = Font::F1;
						}
					}
				'D' => match font {
						Font::F1 => {
							font_str.push_str("04");
						}
						_ => {
							font_str.push_str("|F1,04");
							font = Font::F1;
						}
					}
				'E' => match font {
						Font::F1 => {
							font_str.push_str("05");
						}
						_ => {
							font_str.push_str("|F1,05");
							font = Font::F1;
						}
					}
				'F' => match font {
						Font::F1 => {
							font_str.push_str("06");
						}
						_ => {
							font_str.push_str("|F1,06");
							font = Font::F1;
						}
					}
				'G' => match font {
						Font::F1 => {
							font_str.push_str("07");
						}
						_ => {
							font_str.push_str("|F1,07");
							font = Font::F1;
						}
					}
				'H' => match font {
						Font::F1 => {
							font_str.push_str("08");
						}
						_ => {
							font_str.push_str("|F1,08");
							font = Font::F1;
						}
					}
				'I' => match font {
						Font::F1 => {
							font_str.push_str("09");
						}
						_ => {
							font_str.push_str("|F1,09");
							font = Font::F1;
						}
					}
				'J' => match font {
						Font::F1 => {
							font_str.push_str("0a");
						}
						_ => {
							font_str.push_str("|F1,0a");
							font = Font::F1;
						}
					}
				'K' => match font {
						Font::F1 => {
							font_str.push_str("0b");
						}
						_ => {
							font_str.push_str("|F1,0b");
							font = Font::F1;
						}
					}
				'L' => match font {
						Font::F1 => {
							font_str.push_str("0c");
						}
						_ => {
							font_str.push_str("|F1,0c");
							font = Font::F1;
						}
					}
				'M' => match font {
						Font::F1 => {
							font_str.push_str("0d");
						}
						_ => {
							font_str.push_str("|F1,0d");
							font = Font::F1;
						}
					}
				'N' => match font {
						Font::F1 => {
							font_str.push_str("0e");
						}
						_ => {
							font_str.push_str("|F1,0e");
							font = Font::F1;
						}
					}
				'O' => match font {
						Font::F1 => {
							font_str.push_str("0f");
						}
						_ => {
							font_str.push_str("|F1,0f");
							font = Font::F1;
						}
					}
				'P' => match font {
						Font::F1 => {
							font_str.push_str("10");
						}
						_ => {
							font_str.push_str("|F1,10");
							font = Font::F1;
						}
					}
				'Q' => match font {
						Font::F1 => {
							font_str.push_str("11");
						}
						_ => {
							font_str.push_str("|F1,11");
							font = Font::F1;
						}
					}
				'R' => match font {
						Font::F1 => {
							font_str.push_str("12");
						}
						_ => {
							font_str.push_str("|F1,12");
							font = Font::F1;
						}
					}
				'S' => match font {
						Font::F1 => {
							font_str.push_str("13");
						}
						_ => {
							font_str.push_str("|F1,13");
							font = Font::F1;
						}
					}
				'T' => match font {
						Font::F1 => {
							font_str.push_str("14");
						}
						_ => {
							font_str.push_str("|F1,14");
							font = Font::F1;
						}
					}
				'U' => match font {
						Font::F1 => {
							font_str.push_str("15");
						}
						_ => {
							font_str.push_str("|F1,15");
							font = Font::F1;
						}
					}
				'V' => match font {
						Font::F1 => {
							font_str.push_str("16");
						}
						_ => {
							font_str.push_str("|F1,16");
							font = Font::F1;
						}
					}
				'W' => match font {
						Font::F1 => {
							font_str.push_str("17");
						}
						_ => {
							font_str.push_str("|F1,17");
							font = Font::F1;
						}
					}
				'X' => match font {
						Font::F1 => {
							font_str.push_str("18");
						}
						_ => {
							font_str.push_str("|F1,18");
							font = Font::F1;
						}
					}
				'Y' => match font {
						Font::F1 => {
							font_str.push_str("19");
						}
						_ => {
							font_str.push_str("|F1,19");
							font = Font::F1;
						}
					}
				'Z' => match font {
						Font::F1 => {
							font_str.push_str("1a");
						}
						_ => {
							font_str.push_str("|F1,1a");
							font = Font::F1;
						}
					}
				'a' => match font {
						Font::F1 => {
							font_str.push_str("1b");
						}
						_ => {
							font_str.push_str("|F1,1b");
							font = Font::F1;
						}
					}
				'b' => match font {
						Font::F1 => {
							font_str.push_str("1c");
						}
						_ => {
							font_str.push_str("|F1,1c");
							font = Font::F1;
						}
					}
				'c' => match font {
						Font::F1 => {
							font_str.push_str("1d");
						}
						_ => {
							font_str.push_str("|F1,1d");
							font = Font::F1;
						}
					}
				'd' => match font {
						Font::F1 => {
							font_str.push_str("1e");
						}
						_ => {
							font_str.push_str("|F1,1e");
							font = Font::F1;
						}
					}
				'e' => match font {
						Font::F1 => {
							font_str.push_str("1f");
						}
						_ => {
							font_str.push_str("|F1,1f");
							font = Font::F1;
						}
					}
				'f' => match font {
						Font::F1 => {
							font_str.push_str("20");
						}
						_ => {
							font_str.push_str("|F1,20");
							font = Font::F1;
						}
					}
				'g' => match font {
						Font::F1 => {
							font_str.push_str("21");
						}
						_ => {
							font_str.push_str("|F1,21");
							font = Font::F1;
						}
					}
				'h' => match font {
						Font::F1 => {
							font_str.push_str("22");
						}
						_ => {
							font_str.push_str("|F1,22");
							font = Font::F1;
						}
					}
				'i' => match font {
						Font::F1 => {
							font_str.push_str("23");
						}
						_ => {
							font_str.push_str("|F1,23");
							font = Font::F1;
						}
					}
				'j' => match font {
						Font::F1 => {
							font_str.push_str("24");
						}
						_ => {
							font_str.push_str("|F1,24");
							font = Font::F1;
						}
					}
				'k' => match font {
						Font::F1 => {
							font_str.push_str("25");
						}
						_ => {
							font_str.push_str("|F1,25");
							font = Font::F1;
						}
					}
				'l' => match font {
						Font::F1 => {
							font_str.push_str("26");
						}
						_ => {
							font_str.push_str("|F1,26");
							font = Font::F1;
						}
					}
				'm' => match font {
						Font::F1 => {
							font_str.push_str("27");
						}
						_ => {
							font_str.push_str("|F1,27");
							font = Font::F1;
						}
					}
				'n' => match font {
						Font::F1 => {
							font_str.push_str("28");
						}
						_ => {
							font_str.push_str("|F1,28");
							font = Font::F1;
						}
					}
				'o' => match font {
						Font::F1 => {
							font_str.push_str("29");
						}
						_ => {
							font_str.push_str("|F1,29");
							font = Font::F1;
						}
					}
				'p' => match font {
						Font::F1 => {
							font_str.push_str("2a");
						}
						_ => {
							font_str.push_str("|F1,2a");
							font = Font::F1;
						}
					}
				'q' => match font {
						Font::F1 => {
							font_str.push_str("2b");
						}
						_ => {
							font_str.push_str("|F1,2b");
							font = Font::F1;
						}
					}
				'r' => match font {
						Font::F1 => {
							font_str.push_str("2c");
						}
						_ => {
							font_str.push_str("|F1,2c");
							font = Font::F1;
						}
					}
				's' => match font {
						Font::F1 => {
							font_str.push_str("2d");
						}
						_ => {
							font_str.push_str("|F1,2d");
							font = Font::F1;
						}
					}
				't' => match font {
						Font::F1 => {
							font_str.push_str("2e");
						}
						_ => {
							font_str.push_str("|F1,2e");
							font = Font::F1;
						}
					}
				'u' => match font {
						Font::F1 => {
							font_str.push_str("2f");
						}
						_ => {
							font_str.push_str("|F1,2f");
							font = Font::F1;
						}
					}
				'v' => match font {
						Font::F1 => {
							font_str.push_str("30");
						}
						_ => {
							font_str.push_str("|F1,30");
							font = Font::F1;
						}
					}
				'w' => match font {
						Font::F1 => {
							font_str.push_str("31");
						}
						_ => {
							font_str.push_str("|F1,31");
							font = Font::F1;
						}
					}
				'x' => match font {
						Font::F1 => {
							font_str.push_str("32");
						}
						_ => {
							font_str.push_str("|F1,32");
							font = Font::F1;
						}
					}
				'y' => match font {
						Font::F1 => {
							font_str.push_str("33");
						}
						_ => {
							font_str.push_str("|F1,33");
							font = Font::F1;
						}
					}
				'z' => match font {
						Font::F1 => {
							font_str.push_str("34");
						}
						_ => {
							font_str.push_str("|F1,34");
							font = Font::F1;
						}
					}
				'0' => match font {
						Font::F1 => {
							font_str.push_str("35");
						}
						_ => {
							font_str.push_str("|F1,35");
							font = Font::F1;
						}
					}
				'1' => match font {
						Font::F1 => {
							font_str.push_str("36");
						}
						_ => {
							font_str.push_str("|F1,36");
							font = Font::F1;
						}
					}
				'2' => match font {
						Font::F1 => {
							font_str.push_str("37");
						}
						_ => {
							font_str.push_str("|F1,37");
							font = Font::F1;
						}
					}
				'3' => match font {
						Font::F1 => {
							font_str.push_str("38");
						}
						_ => {
							font_str.push_str("|F1,38");
							font = Font::F1;
						}
					}
				'4' => match font {
						Font::F1 => {
							font_str.push_str("39");
						}
						_ => {
							font_str.push_str("|F1,39");
							font = Font::F1;
						}
					}
				'5' => match font {
						Font::F1 => {
							font_str.push_str("3a");
						}
						_ => {
							font_str.push_str("|F1,3a");
							font = Font::F1;
						}
					}
				'6' => match font {
						Font::F1 => {
							font_str.push_str("3b");
						}
						_ => {
							font_str.push_str("|F1,3b");
							font = Font::F1;
						}
					}
				'7' => match font {
						Font::F1 => {
							font_str.push_str("3c");
						}
						_ => {
							font_str.push_str("|F1,3c");
							font = Font::F1;
						}
					}
				'8' => match font {
						Font::F1 => {
							font_str.push_str("3d");
						}
						_ => {
							font_str.push_str("|F1,3d");
							font = Font::F1;
						}
					}
				'9' => match font {
						Font::F1 => {
							font_str.push_str("3e");
						}
						_ => {
							font_str.push_str("|F1,3e");
							font = Font::F1;
						}
					}
				' ' => match font {
						Font::F1 => {
							font_str.push_str("3f");
						}
						_ => {
							font_str.push_str("|F1,3f");
							font = Font::F1;
						}
					}
				'.' => match font {
						Font::F1 => {
							font_str.push_str("41");
						}
						_ => {
							font_str.push_str("|F1,41");
							font = Font::F1;
						}
					}
				_ => {font = Font::F1; symbol = "".to_owned()}
			}
		}
	}
	font_str.remove(0);
	println!("font str: {}", font_str);
	font_str
}

//#[allow(dead_code)]
//fn demos() {
//	let args: Vec<String> = std::env::args().collect();
//	adding_new_content_object::run_demo(&args[1], args[2].clone());
//	embed_fonts_example::run_demo();
//	return;
//}
