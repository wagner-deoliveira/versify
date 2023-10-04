use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn read_file(path_to_file: &str) -> String {
    let mut file_lines = String::new();
    let mut file = File::open(path_to_file).unwrap_or_else(|error| {
        eprint!("Failed to open file: {error}");
        exit(1)
    });

    file.read_to_string(&mut file_lines).unwrap();

    return file_lines;
}
