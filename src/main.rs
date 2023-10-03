mod args;
mod mappings;

use regex::Regex;
use std::fs::File;
use std::{fs, io};
use std::collections::HashMap;
use std::io::Read;
use std::process::exit;
use clap::Parser;
use args::VersifyArgs;
use crate::mappings::App;

fn replace_version(file: &String, map: HashMap<&str, &str>) -> io::Result<()> {
    fs::create_dir_all("output")?;
    let output_file_path = "output/packages.txt";
    let mut modified_file = String::new();

    for (key, value) in &map {
        let binding = Regex::new(&key).unwrap();
        let app = binding.as_str();

        match app.parse::<App>() {
            Ok(x) => {
                for line in file.lines() {
                    let domain_space = mappings::inspect_app(x);
                    let mut modified_line = line.to_string();

                    let binding_args = Regex::new(r":(main/latest|\d+(\.\d+){0,3})");

                    if domain_space.contains(&line) {
                        if binding_args.expect("REASON").is_match(&modified_line) {
                            modified_line = modified_line.replace(":main/latest", &format!(":{:}", &value));
                        }
                    }

                    if line != modified_file {
                        modified_file.push_str(&modified_line);
                        modified_file.push('\n');
                    }
                }
            }
            Err(_) => println!("Invalid value, please check your configuration")
        }
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


fn main() -> io::Result<()> {
    let args = VersifyArgs::parse();
    let path = args.path;
    let domains = args.domain;
    let versions = args.build_number;

    let domain_list: Vec<&str> = domains.split(",").collect();
    let version_list: Vec<&str> = versions.split(",").collect();

    if domain_list.len() != version_list.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Domain_list and version_list have different sizes. \
            Make sure to enter the same number of domains and versions",
        ));
    };

    let version_mapping: HashMap<&str, &str> = domain_list.clone().into_iter().zip(version_list.clone().into_iter()).collect();

    let file = read_file(&*path);
    replace_version(&file, version_mapping)
}
