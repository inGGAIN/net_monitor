use std::fs::OpenOptions;
use std::io::Write;

pub fn save_log(text: &str) {
    let file_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open("network_log.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Gagal membuka file log: {}", error);
            return;
        }
    };

    if let Err(error) = writeln!(file, "{}", text) {
        eprintln!("Gagal menulis log: {}", error);
    }
}
