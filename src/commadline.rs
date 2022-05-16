use lopdf::Object;
use serde::{Deserialize, Serialize};
use std::fmt;

use std::error::Error;
use std::str::FromStr;

use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser, Debug, Clone)]
#[clap(author, version = "0.0.3-alpha", about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub task: Tasks,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Tasks {
    /// edit pdf file, pdf file must be patched first, using a non-patched pdf will result in
    /// undefined behavior
    #[clap(group(ArgGroup::new("oprs").required(true).args(&[ "operations", "opr-file","undo"])))]
    Edit {
        /// run `leafedit list operations` for a list of supported operations
        #[clap(short)]
        operations: Vec<Opr>,
        /// undo last edit command
        #[clap(long)]
        undo: bool,
        /// use file of operations
        #[clap(short = 'f')]
        opr_file: Option<String>,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT", default_value = "out.pdf")]
        save_as: String,
    },

    Grid {
        /// grid type
        #[clap(subcommand)]
        gridtype: GridType,
    },

    /// apply a set of operations that are necessary before any edits,
    /// like embeding fonts and reseting content tranformation matrix
    Patch {
        /// page to patch
        #[clap(short, default_value = "1")]
        page: u32,
        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,
        /// /out/file/path
        #[clap(name = "OUTPUT")]
        save_as: String,
    },

    Merge {
        /// /path/to/files
        #[clap(name = "INPUTS", required = true, multiple_occurrences = true)]
        files: Vec<String>,

        /// /out/file/path
        #[clap(name = "OUTPUT", required = true)]
        save_as: String,
    },

    /// list options for different commands
    List {
        #[clap(subcommand)]
        list: ListOptions,
    },

    /// show information about pdf
    Info {
        /// what information to show
        #[clap(subcommand)]
        about: Information,

    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Information {
    PageSize{
        /// page to show size of
        #[clap(short, default_value = "1")]
        page: u32,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum GridType {
    /// grid the entire page with an interval 20 points
    Full {
        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT", default_value = "out.pdf")]
        save_as: String,
    },

    /// grid with interval 4 around intersection point from full grid
    Sub {
        /// number of the horizantal line
        x: u32,

        /// number of the vertial line
        y: u32,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT", default_value = "out.pdf")]
        save_as: String,
    },

    /// mark a point from the at postion x, y in the pdf graph
    Mark {
        /// x-coordinate in pdf content graph
        x: u32,

        /// y-coordinate in pdf content graph
        y: u32,

        /// make printed text vertical
        #[clap(short = 'r')]
        rotate: bool,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT", default_value = "out.pdf")]
        save_as: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListOptions {
    /// list supported options
    Operations,
    /// list supported grid types
    GridType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Opr {
    #[serde(rename = "Wr")]
    WriteLine (u32, u32, u32, String),

    #[serde(rename = "Cc")]
    ChangeColor (Color),

    #[serde(rename = "Crgb")]
    ChangeRgb(u8, u8, u8),

    #[serde(rename = "Lw")]
    SetWidth (u32),

    #[serde(rename = "Dl")]
    DrawLine (u32, u32, u32, u32),

    #[serde(skip)]
    Raw(String, Vec<Object>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "grey")]
    Grey,
}

impl Error for ParseOperationError {}

impl fmt::Display for ParseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid operation, run `leafedit list operations` \
               to get list of supported operations")
    }
}

#[derive(Debug)]
pub struct ParseOperationError;

impl FromStr for Opr {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ron::from_str(s) {
            Ok(opr) => Ok(opr),
            Err(_) => Err(ParseOperationError),
        }
    }

}
