use serde_json::{json, Value};
use std::fs::File;
use std::io;
use log::info;

pub struct Reporter<'a> {
    target_hash: &'a str,
    algorithm: &'a str,
    result: &'a Option<String>,
}

impl<'a> Reporter<'a> {
    pub fn new(target_hash: &'a str, algorithm: &'a str, result: &'a Option<String>) -> Self {
        Reporter {
            target_hash,
            algorithm,
            result,
        }
    }

    pub fn save_report(&self, output_path: &str) -> io::Result<()> {
        let report = json!({
            "target_hash": self.target_hash,
            "algorithm": self.algorithm,
            "password_found": self.result.as_ref().map_or("None".to_string(), |p| p.to_string()),
            "status": if self.result.is_some() { "Success" } else { "Failed" }
        });
        let file = File::create(output_path)?;
        serde_json::to_writer_pretty(file, &report)?;
        info!("Report saved to {}", output_path);
        Ok(())
    }
}
