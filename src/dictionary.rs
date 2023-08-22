use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::io::{self};

use crate::arguments::Args;
use crate::parsing::{build_longform, key_swap, new_longform};

pub type KeyPair = BTreeMap<String, String>;

pub enum Mode {
    FindAbbreviation,
    FindLongForm,
}

impl Mode {
    pub fn next(&mut self) {
        match self {
            Mode::FindAbbreviation => &mut Mode::FindLongForm,
            Mode::FindLongForm => &mut Mode::FindAbbreviation,
        };
    }
}

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
}

pub fn parse_inputs() -> Result<Dictionary> {
    Dictionary::from_arguments(Args::parse())
}

pub fn layer_lookup(dictionary: &mut Dictionary) -> bool {
    loop {
        //body
        println!("Under construction");
        //conclusion
        let input = io::stdin()
            .lines()
            .next()
            .unwrap_or(Ok(String::new()))
            .unwrap_or(String::new());
        match input.trim() {
            ":q" => return false,
            ":s" => {
                dictionary.mode.next();
            }
            _ => (),
        };
    }
}
