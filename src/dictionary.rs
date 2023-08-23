use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::io::{self};

use crate::arguments::Args;
use crate::parsing::{build_longform, key_swap, new_longform};

// pub type KeyPair = Vec<(String, Vec<String>)>;
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
        let list = match self.mode {
            Mode::FindAbbreviation => &self.to_abbreviation,
            Mode::FindLongForm => &self.to_longform,
        };
        let mut hits = 0;

        if let Some(w) = self.perfect_search(list) {
            println!("Exact Match: {}", w);
            hits += 1;
        }
        if hits == 0 {
            println!("No relevant definitions.")
        }
    }

    fn perfect_search(&self, list: &KeyPair) -> Option<String> {
        list.get(&self.search_term).cloned()
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
