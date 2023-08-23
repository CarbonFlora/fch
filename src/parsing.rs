use anyhow::Result;
use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::dictionary::KeyPair;

const DIR: &str = ".\\CAD_LAYER_DICTIONARY.ron";

pub fn new_longform(file_path: &str) -> Result<KeyPair> {
    let mut map = KeyPair::new();
    let mut buffered = BufReader::new(File::open(file_path)?)
        .lines()
        .flatten()
        .peekable();
    let mut search_key = String::new();

    while let Some(line) = buffered.next() {
        if line.starts_with(':') {
            search_key = line.strip_prefix(':').unwrap_or_default().to_string();
        } else if line.starts_with(&search_key) {
            let key = line.split_whitespace().next().unwrap_or_default().trim();
            let mut value = line.strip_prefix(key).unwrap_or_default().to_string();

            while buffered
                .peek()
                .is_some_and(|x| !x.starts_with(&search_key) && !x.starts_with(':'))
            {
                value += &buffered.next().unwrap_or_default();
            }

            map.insert(key.to_string(), value.trim().to_string());
        }
    }

    write_ron(&map)?;
    Ok(map)
}

pub fn build_longform() -> Result<KeyPair> {
    // let mut directory = dirs::home_dir().unwrap_or_default();
    // directory.push(fs::canonicalize(DIR)?);
    let decode = std::fs::read_to_string(DIR)?;
    let map: KeyPair = from_str(&decode)?;

    Ok(map)
}

pub fn key_swap(keypair: KeyPair) -> KeyPair {
    let mut rev_keypair = KeyPair::new();
    for pair in keypair.iter() {
        rev_keypair.insert(pair.1.to_string(), pair.0.to_string());
    }

    rev_keypair
}

pub fn write_ron(keypair: &KeyPair) -> Result<()> {
    let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let s = to_string_pretty(&keypair, pretty)?;
    // let mut directory = dirs::home_dir().unwrap_or_default();
    // directory.push(DIR);
    // let mut file_path = directory.clone();
    // file_path.push("CAD_LAYER_DICTIONARY.ron");

    // fs::create_dir_all(directory)?;
    write!(File::create(DIR)?, "{}", s)?;
    Ok(())
}
