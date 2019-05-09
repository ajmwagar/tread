#![feature(duration_float)]
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use std::error::Error;
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use termion::event::{Key, MouseEvent};
use termion::raw::IntoRawMode;
use termion::event::*;
use termion::cursor;
use termion::input::{TermRead, MouseTerminal};
use std::io::{self, Write};


use std::{thread, time::Duration};

/// State of the Reader
pub struct State {
    wpm: f64,
    hide: bool,
    pause: bool,
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "tread")]
pub struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

/// Read file word by word
pub struct SpeedReader {
    pub index: usize,
    pub words: Vec<String>,
}

impl SpeedReader {
    /// Create a new speeed reader from a path to a file
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path.clone())?;

        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let words = contents.clone().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();

        Ok(SpeedReader {
            index: 0,
            words: words,
        })

    }

    /// Move the index up by one
    pub fn next(&mut self) {
        if self.index < self.words.len() {
            self.index += 1;
        }
    }

    /// Move the index back by one
    pub fn prev(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        }
    }

    /// Return the current word
    pub fn get_word(&self) -> String {
        if self.index < self.words.len() {
            self.words[self.index].clone()
        }
        else {
            return "FIN".to_string();
        }
    }
}
