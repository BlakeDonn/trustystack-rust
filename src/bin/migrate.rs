use dotenv::dotenv;
use rust_backend::migration::run;
use std::process;

fn main() {
    // Load .env file before anything else
    dotenv().ok();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
