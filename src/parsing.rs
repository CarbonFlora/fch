use anyhow::Result;

use crate::dictionary::KeyPair;

pub fn new_longform(path: &str) -> Result<KeyPair> {
    let map = KeyPair::new();

    Ok(map)
}

pub fn build_longform() -> Result<KeyPair> {
    let map = KeyPair::new();

    Ok(map)
}

pub fn key_swap(keypair: KeyPair) -> KeyPair {
    keypair
}
