use crate::SECRET;
use hex::ToHex;
use sha2::{Digest, Sha256};
use sqlite::Connection;

pub fn hash(to_hash: String) -> String {
    let mut hasher: Sha256 = Digest::new();
    hasher.update(to_hash);
    (&hasher.finalize()[..]).encode_hex::<String>()
}

pub fn generate_token(username: String) -> String {
    let secret = SECRET.to_string();
    hash(username + &secret)
}

pub fn check_auth(token: String) -> Option<String> {
    let connection: Connection = sqlite::open("data.db").unwrap();
    let statement = connection.prepare(format!(
        "SELECT username FROM User WHERE current_token = '{}'",
        token
    ));
    if statement.is_err() {
        println!("DBError: {}", statement.err().unwrap().message.unwrap());
        return None;
    }
    let mut statement = statement.unwrap();
    if statement.next().is_err() {
        return None;
    }
    let username = statement.read::<String>(0);
    match username {
        Ok(_) => return Some(username.unwrap()),
        Err(_) => return None,
    }
}
