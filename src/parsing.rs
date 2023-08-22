use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::dictionary::KeyPair;

pub fn new_longform(file_path: &str) -> Result<KeyPair> {
    let map = KeyPair::new();
    let buffered = BufReader::new(File::open(file_path)?)
        .lines()
        .flatten()
        .peekable();

    for line in buffered {
        if line.starts_with(':') {
            let left = line.split_whitespace()
        }
    }

    Ok(map)
}

pub fn build_longform() -> Result<KeyPair> {
    let map = KeyPair::new();

    Ok(map)
}

pub fn key_swap(keypair: KeyPair) -> KeyPair {
    let mut rev_keypair = KeyPair::new();
    for pair in keypair.iter() {
        rev_keypair.insert(pair.1.to_string(), pair.0.to_string());
    }

    rev_keypair
}
