use std::fs::OpenOptions;
use std::io::Write;

pub fn log(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("libry.log")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}
