use chrono::Local;

pub fn format_domain_log(domain: &str, status: &str, source: &str) -> String {
    let time = Local::now().format("%H:%M:%S").to_string();

    format!(
        "[{}] STATUS={} DOMAIN={} SOURCE={}",
        time,
        status,
        domain,
        source
    )
}
