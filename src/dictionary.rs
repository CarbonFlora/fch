use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::io::{self};

use crate::arguments::Args;
use crate::parsing::{build_longform, key_swap, new_longform};

pub type KeyPair = BTreeMap<String, String>;

#[derive(Debug)]
pub enum Mode {
    FindAbbreviation,
    FindLongForm,
}

#[derive(Debug)]
pub struct Dictionary {
    to_abbreviation: KeyPair,
    to_longform: KeyPair,
    mode: Mode,
    search_term: String,
}

impl Dictionary {
    pub fn from_arguments(arguments: Args) -> Result<Self> {
        let (to_longform, search_term) = match arguments.build {
            true => (new_longform(&arguments.input)?, String::new()),
            false => (build_longform()?, arguments.input),
        };
        let to_abbreviation = key_swap(to_longform.clone());
        let mode = match arguments.short {
            false => Mode::FindAbbreviation,
            true => Mode::FindLongForm,
        };

        Ok(Dictionary {
            to_abbreviation,
            to_longform,
            mode,
            search_term,
        })
    }

    pub fn layer_lookup(&mut self) -> bool {
        loop {
            //body
            match self.search_term.trim() {
                ":q" => return false,
                ":s" => {
                    self.switch_mode();
                }
                ":debug" => {
                    println!("{:#?}", self);
                }
                "" => println!("Quit with [:q], switch modes with [:s], type anything to search."),
                _ => self.search(),
            }

            //re-search
            self.search_term = io::stdin()
                .lines()
                .next()
                .unwrap_or(Ok(String::new()))
                .unwrap_or(String::new());
        }
    }

    fn search(&self) {
        match self.mode {
            Mode::FindAbbreviation => println!("ABBREVIATION LIST: {:#?}", self.to_abbreviation),
            Mode::FindLongForm => println!("LONGFORM LIST: {:#?}", self.to_longform),
        }
    }

    fn switch_mode(&mut self) {
        self.mode = match self.mode {
            Mode::FindAbbreviation => {
                println!("Ex. CG => Civil Grading");
                Mode::FindLongForm
            }
            Mode::FindLongForm => {
                println!("Ex. Civil Grading => CG");
                Mode::FindAbbreviation
            }
        };
    }
}

pub fn parse_inputs() -> Result<Dictionary> {
    Dictionary::from_arguments(Args::parse())
}
