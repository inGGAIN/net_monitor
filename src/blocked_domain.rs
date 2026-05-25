use std::fs;

pub fn load_blocked_domains(file_path: &str) -> Vec<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Gagal membaca file blocked domains: {}", error);
            return Vec::new();
        }
    };

    content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .collect()
}

pub fn is_blocked_domain(domain: &str, blocked_domains: &[String]) -> bool {
    blocked_domains.iter().any(|blocked| {
        domain == blocked || domain.ends_with(&format!(".{}", blocked))
    })
}
