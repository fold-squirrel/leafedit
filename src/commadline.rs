use serde::{Deserialize, Serialize};
use std::fmt;

use std::error::Error;
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version = "0.1", about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub task: Tasks,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Tasks {
    /// edit pdf file, pdf file must be patched first, using a non-patched pdf will result in
    /// undefined behavior
    Edit {
        /// page size, required for correct font size rendering
        #[clap(short, default_value = "Word")]
        page_size: PageSize,
        /// run `leafedit list operations` for a list of supported operations
        #[clap(short, required = true)]
        operations: Vec<Opr>,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT")]
        save_as: String,
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

    /// list options for different commands
    List {
        #[clap(subcommand)]
        list: ListOptions,
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListOptions {
    /// list supported page sizes
    PageSize,
    /// list supported options
    Operations
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PageSize {
    Word,
    A4,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Opr {
    #[serde(rename = "Wr")]
    WriteLine {
        #[serde(rename = "x")]
        px: u32,
        #[serde(rename = "y")]
        py: u32,
        #[serde(rename = "f")]
        size: u32,
        #[serde(rename = "t")]
        text: String
    }
}

impl Error for ParseOperationError {}
impl Error for PageSizeNotUndersoodError {}

impl fmt::Display for ParseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid operation, run `leafedit list operations` \
               to get list of supported operations")
    }
}

impl fmt::Display for PageSizeNotUndersoodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Page Size not supported, `leafedit list pagesizes` \
               to get list of supported sizes")
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

#[derive(Debug)]
pub struct PageSizeNotUndersoodError;
impl FromStr for PageSize {
    type Err = PageSizeNotUndersoodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ron::from_str(s) {
            Ok(opr) => Ok(opr),
            Err(_) => Err(PageSizeNotUndersoodError),
        }
    }

}

