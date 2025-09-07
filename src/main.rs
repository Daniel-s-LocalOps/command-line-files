use chrono::Local;
use std::io::prelude::*;
use std::{env, fs};

fn main() {
    let file_name = get_filename_from_enviroment();
    let contents = read_file_contents(&file_name);

    copy_file(&file_name, &contents);
    append_to_file(&file_name, "\nthis is another name");
}

fn get_filename_from_enviroment() -> String {
    // env::args at 0 is the program name, which isn't helpful
    const FIRST_ARGUEMENT: usize = 1;

    env::args().nth(FIRST_ARGUEMENT).unwrap_or_else(|| {
        eprintln!("Please enter a file name");
        std::process::exit(1);
    })
}

fn read_file_contents(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap_or_else(|e| {
        eprintln!("An error occured while reading the file: {}", e);
        std::process::exit(1);
    })
}

fn append_to_file(file_name: &str, append_text: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(file_name)
        .unwrap_or_else(|e| {
            eprintln!("An error occured while writing file: {}", e);
            std::process::exit(1);
        });

    file.write(append_text.as_bytes()).unwrap_or_else(|e| {
        eprintln!("An error occured appending to file: {}", e);
        std::process::exit(1);
    });
}

fn copy_file(file_name: &str, content: &str) {
    let file_name_copy = append_datetime_to_file_name(file_name);

    fs::write(file_name_copy, content).unwrap_or_else(|e| {
        eprintln!("An error occured while copying file: {}", e);
        std::process::exit(1);
    })
}

fn append_datetime_to_file_name(file_name: &str) -> String {
    let file_name = file_name
        .split_once('.')
        .map(|(before, _)| before)
        .unwrap_or(file_name);

    format!(
        "{}-{}.txt",
        file_name,
        Local::now().format("%Y-%m-%d-%H:%M:%S").to_string()
    )
}
