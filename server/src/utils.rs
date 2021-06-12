use crate::SECRET;
use hex::ToHex;
use sha2::{Digest, Sha256};

pub fn hash(to_hash: String) -> String {
    let mut hasher: Sha256 = Digest::new();
    hasher.update(to_hash);
    (&hasher.finalize()[..]).encode_hex::<String>()
}

pub fn generate_token(username: String) -> String {
    let secret = SECRET.to_string();
    hash(username + &secret)
}
