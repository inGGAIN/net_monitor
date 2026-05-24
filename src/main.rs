use chrono::Local;
use std::io::{self, BufRead};

mod blocked_domain;

fn main() {
    println!("Realtime Network Monitor");
    println!("Membaca data dari stdin...\n");

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Read error: {}", error);
                continue;
            }
        };

        if let Some(domain) = extract_domain(&text) {
            print_domain(&domain);
        }
    }
}

fn extract_domain(line: &str) -> Option<String> {
    let words: Vec<&str> = line.split_whitespace().collect();

    for word in words {
        if is_possible_domain(word) {
            return Some(clean_domain(word));
        }
    }

    None
}

fn is_possible_domain(text: &str) -> bool {
    text.contains('.') &&
    !text.contains('/') &&
    !text.contains(':') &&
    text.len() > 3
}

fn clean_domain(text: &str) -> String {
    text.trim()
        .trim_matches(',')
        .trim_matches(';')
        .trim_matches(')')
        .trim_matches('(')
        .to_string()
}

fn print_domain(domain: &str) {
    let time = Local::now().format("%H:%M:%S").to_string();

    if blocked_domain::is_blocked_domain(domain) {
        println!("[{}] BLOCKED_DOMAIN {}", time, domain);
    } else {
        println!("[{}] ALLOWED_DOMAIN {}", time, domain);
    }
}
