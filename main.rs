```rust
use clap::{Arg, Command};
use log::info;
use rust_pass_cracker::cracker::{md5_cracker::MD5Cracker, sha256_cracker::SHA256Cracker};
use rust_pass_cracker::report::reporter::Reporter;
use rust_pass_cracker::utils::logger::init_logger;

fn main() {
    // Initialize logger
    init_logger();

    // Ethical use disclaimer
    info!("RustPassCracker: For educational purposes only. Use only with explicit permission.");

    // Parse command-line arguments
    let matches = Command::new("RustPassCracker")
        .version("0.1.0")
        .author("Your Name")
        .about("A password cracker for educational purposes")
        .arg(
            Arg::new("hash")
                .help("Target hash to crack")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("wordlist")
                .help("Path to wordlist file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("algo")
                .help("Hash algorithm (md5 or sha256)")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("output")
                .help("Output JSON report file")
                .default_value("report.json"),
        )
        .get_matches();

    let target_hash = matches.get_one::<String>("hash").unwrap();
    let wordlist_path = matches.get_one::<String>("wordlist").unwrap();
    let algo = matches.get_one::<String>("algo").unwrap();
    let output = matches.get_one::<String>("output").unwrap();

    // Initialize result
    let mut result = None;

    // Select cracker based on algorithm
    match algo.as_str() {
        "md5" => {
            let cracker = MD5Cracker::new(target_hash, wordlist_path);
            result = cracker.crack();
        }
        "sha256" => {
            let cracker = SHA256Cracker::new(target_hash, wordlist_path);
            result = cracker.crack();
        }
        _ => {
            eprintln!("Unsupported algorithm: {}. Use 'md5' or 'sha256'.", algo);
            std::process::exit(1);
        }
    }

    // Generate report
    let reporter = Reporter::new(target_hash, algo, &result);
    if let Err(e) = reporter.save_report(output) {
        eprintln!("Failed to save report: {}", e);
    } else {
        info!("Report saved to {}", output);
    }
}
```
