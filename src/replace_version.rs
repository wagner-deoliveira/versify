use std::collections::HashMap;
use std::fs;
use std::io::Error;
use regex::Regex;
use crate::mappings;
use crate::mappings::App;

pub fn replace_version(file: &String, map: HashMap<&str, &str>, output_path: &str) -> Result<(), Error> {
    let mut modified_contents = file.clone();

    for (key, value) in &map {
        let binding = Regex::new(&key).unwrap();
        let app = binding.as_str();

        match app.parse::<App>() {
            Ok(x) => {
                let domain_space = mappings::inspect_app(x);
                let mut found_string;

                for line in &domain_space {
                    for mod_line in modified_contents.clone().lines() {
                        if mod_line.starts_with(line) {
                            found_string = mod_line;
                            modified_contents = modified_contents.replace(found_string, &format!("{}:{:}", &line, value));
                        }
                    }
                }
            }
            Err(_) => println!("Invalid value, please check your configuration")
        }
    }
    fs::create_dir_all(output_path)?;
    let output_file_path = &format!("{}/packages.txt", output_path);
    fs::write(output_file_path, &modified_contents).expect("Unable to write to output file");

    Ok(())
}
