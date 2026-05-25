use chrono::Local;
use std::io::{self, BufRead};

mod blocked_domain;
mod domain_parser;
mod logger;

fn main() {
    println!("Realtime Network Monitor");
    println!("Membaca data dari stdin...");
    println!("Contoh input:");
    println!("query A youtube.com");
    println!("connect to api.whatsapp.com");
    println!("Tekan CTRL + C untuk keluar.\n");

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Read error: {}", error);
                continue;
            }
        };

        if let Some(domain) = domain_parser::extract_domain(&text) {
            print_domain(&domain);
        }
    }
}

fn print_domain(domain: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();

    let output = if blocked_domain::is_blocked_domain(domain) {
        format!("[{}] BLOCKED_DOMAIN {}", time, domain)
    } else {
        format!("[{}] ALLOWED_DOMAIN {}", time, domain)
    };

    println!("{}", output);
    logger::save_log(&output);
}
