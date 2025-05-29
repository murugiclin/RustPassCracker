use log::{info, error};
use sha2::{Digest, Sha256};
use crate::utils::file_reader::read_wordlist;

pub struct SHA256Cracker {
    target_hash: String,
    wordlist_path: String,
}

impl SHA256Cracker {
    pub fn new(target_hash: &str, wordlist_path: &str) -> Self {
        SHA256Cracker {
            target_hash: target_hash.to_string(),
            wordlist_path: wordlist_path.to_string(),
        }
    }

    pub fn crack(&self) -> Option<String> {
        info!("Starting SHA-256 cracking for hash: {}", self.target_hash);
        match read_wordlist(&self.wordlist_path) {
            Ok(wordlist) => {
                for word in wordlist {
                    let word = word.trim();
                    let mut hasher = Sha256::new();
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
