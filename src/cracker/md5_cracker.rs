use log::{info, error};
use md5::{Digest, Md5};
use crate::utils::file_reader::read_wordlist;

pub struct MD5Cracker {
    target_hash: String,
    wordlist_path: String,
}

impl MD5Cracker {
    pub fn new(target_hash: &str, wordlist_path: &str) -> Self {
        MD5Cracker {
            target_hash: target_hash.to_string(),
            wordlist_path: wordlist_path.to_string(),
        }
    }

    pub fn crack(&self) -> Option<String> {
        info!("Starting MD5 cracking for hash: {}", self.target_hash);
        match read_wordlist(&self.wordlist_path) {
            Ok(wordlist) => {
                for word in wordlist {
                    let word = word.trim();
                    let mut hasher = Md5::new();
                    hasher.update(word);
                    let result = format!("{:x}", hasher.finalize());
                    if result == self.target_hash {
                        info!("Password found: {}", word);
                        return Some(word.to_string());
                    }
                }
                info!("No password found for hash: {}", self.target_hash);
                None
            }
            Err(e) => {
                error!("Failed to read wordlist: {}", e);
                None
            }
        }
    }
}
