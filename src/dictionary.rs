use anyhow::Result;
use clap::Parser;
use rust_fuzzy_search::fuzzy_search_threshold;
use std::collections::BTreeMap;
use std::io::{self};

use crate::arguments::Args;
use crate::parsing::{key_swap, new_longform};

pub type KeyPair = BTreeMap<String, String>;
const THRESHOLD: f32 = 0.4;

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
        let to_longform = new_longform(&arguments.dictionary)?;
        let search_term = String::new();
        let to_abbreviation = key_swap(to_longform.clone());
        let mode = Mode::FindAbbreviation;

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
                .unwrap_or(String::new())
                .to_lowercase();
        }
    }

    fn search(&self) {
        let list = match self.mode {
            Mode::FindAbbreviation => &self.to_abbreviation,
            Mode::FindLongForm => &self.to_longform,
        };

        if let Some(w) = self.perfect_search(list) {
            println!("\n[Exact Match]{}", w);
        }
        if let Some(w) = self.fuzzy_search(list) {
            println!("\n[Fuzzy Matches]{}", w);
        } else {
            println!("No relevant definitions.");
        }
    }

    fn perfect_search(&self, list: &KeyPair) -> Option<String> {
        let value = list.get(&self.search_term).cloned()?;

        Some(format!("\n{} => {}", self.search_term, value))
    }

    fn fuzzy_search(&self, list: &KeyPair) -> Option<String> {
        let keys = list.keys().cloned().collect::<Vec<String>>();
        let mut within_threshold = String::new();

        for i in keys {
            let parts = i.split([' ', '-']).collect::<Vec<&str>>();
            let binding = fuzzy_search_threshold(&self.search_term, &parts, THRESHOLD);
            // let _ = binding
            //     .iter()
            //     .map(|x| within_threshold += format!("\n{}", x.0).as_str());
            if !binding.is_empty() {
                let value = list.get(&i).unwrap(); //unreachable unwrap.
                within_threshold += format!("\n{} => {}", i, value).as_str();
            }
        }

        if within_threshold.is_empty() {
            return None;
        }
        Some(within_threshold)
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
