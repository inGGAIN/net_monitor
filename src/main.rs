use std::io::{self, BufRead};

mod blocked_domain;
mod domain_parser;
mod logger;
mod output_formatter;

fn main() {
    println!("Realtime Network Monitor");
    println!("Membaca data dari stdin...");
    println!("Program ini siap menerima input manual atau input dari pipe.");
    println!();

    let blocked_domains = blocked_domain::load_blocked_domains("blocked_domains.txt");

    println!("Loaded {} blocked domains", blocked_domains.len());
    println!();
    println!("Contoh manual:");
    println!("query A youtube.com");
    println!();
    println!("Contoh pipe:");
    println!("cat sample_dns.log | cargo run");
    println!();
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
            process_domain(&domain, &blocked_domains);
        }
    }
}

fn process_domain(domain: &str, blocked_domains: &[String]) {
    let status = if blocked_domain::is_blocked_domain(domain, blocked_domains) {
        "BLOCKED"
    } else {
        "ALLOWED"
    };

    let output = output_formatter::format_domain_log(domain, status, "stdin");

    println!("{}", output);
    logger::save_log(&output);
}
