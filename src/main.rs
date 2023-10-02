mod args;
mod mappings;

use regex::Regex;
use std::fs::File;
use std::{fs, io};
use std::io::Read;
use std::process::exit;
use clap::Parser;
use args::VersifyArgs;
use crate::mappings::App;

fn replace_version<'a>(file: &'a String, domain_app: &'a str, version_to_change: &'a str) -> io::Result<()> {
    let binding = Regex::new(&domain_app).unwrap();
    let app = binding.as_str();
    let mut modified_file = String::new();

    fs::create_dir_all("output")?;
    let output_file_path = "output/packages.txt";

    match app.parse::<App>() {
        Ok(x) => {
            for line in file.lines() {
                let domain_space = mappings::inspect_app(x);
                let mut modified_line = line.to_string();

                let binding_args = Regex::new(r":(main/latest|\d+(\.\d+){0,3})");

                if domain_space.contains(&line) {
                    if binding_args.expect("REASON").is_match(&modified_line) {
                        modified_line = modified_line.replace(":main/latest", &format!(":{}", version_to_change));
                    }
                }
                modified_file.push_str(&modified_line);
                modified_file.push('\n');
            }
        }
        Err(_) => println!("Invalid value")
    }

    fs::write(output_file_path, &modified_file).expect("Unable to write to output file");

    Ok(())
}

fn read_file(path_to_file: &str) -> String {
    let mut file_lines = String::new();
    let mut file = File::open(path_to_file).unwrap_or_else(|error| {
        eprint!("Failed to open file: {error}");
        exit(1)
    });

    file.read_to_string(&mut file_lines).unwrap();

    return file_lines;
}


fn main() {
    let args = VersifyArgs::parse();
    let path = args.path;
    let domains = args.domain;
    let versions = args.build_number;

    let domain_list: Vec<&str> = domains.split(",").collect();
    let version_list: Vec<&str> = versions.split(",").collect();

    let file = read_file(&*path);

    let domain_app = domain_list.get(0).unwrap();

    for v in version_list.iter() {
        println!("{:?}", replace_version(&file, domain_app, v));
    }
}
