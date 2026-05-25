pub fn extract_domain(line: &str) -> Option<String> {
    let words: Vec<&str> = line.split_whitespace().collect();

    for word in words {
        if is_possible_domain(word) {
            return Some(clean_domain(word));
        }
    }

    None
}

fn is_possible_domain(text: &str) -> bool {
    text.contains('.')
        && !text.contains('/')
        && !text.contains(':')
        && text.len() > 3
}

fn clean_domain(text: &str) -> String {
    text.trim()
        .trim_matches(',')
        .trim_matches(';')
        .trim_matches(')')
        .trim_matches('(')
        .to_string()
}
