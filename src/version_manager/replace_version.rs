use std::collections::HashMap;
use std::fs;
use std::error::Error;
use regex::Regex;
use crate::version_manager::mappings;

pub fn replace_version_to_file(file: &str, map: HashMap<&str, &str>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_contents = file;

    let replace_fn = |line: &str| {
        let mut modified_line = String::from(line);

        for (key, value) in &map {
            let binding = Regex::new(key).unwrap();
            let app = binding.as_str();

            match app.parse::<mappings::App>() {
                Ok(app) => {
                    let domain_space = mappings::inspect_app(&app);

                    for domain in &domain_space {
                        if modified_line.starts_with(domain) {
                            let parts: Vec<&str> = modified_line.split(":").collect();
                            modified_line = format!("{}:{}", parts[0], value);
                            break;
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Some of the domains doesn't match a valid value, please check your configuration {:?}", app);
                    std::process::exit(1);
                },
            }
        }

        modified_line
    };

    let modified_contents: String = input_contents
        .lines()
        .map(replace_fn)
        .collect::<Vec<String>>()
        .join("\n");

    fs::create_dir_all(output_path)?;

    let output_file_path = format!("{}/packages.txt", output_path);
    fs::write(&output_file_path, &modified_contents)?;

    Ok(())
}

pub fn replace_version_in_repository(file: &str, map: HashMap<&str, &str>) -> String {
    let input_contents = file;

    let replace_fn = |line: &str| {
        let mut modified_line = String::from(line);

        for (key, value) in &map {
            let binding = Regex::new(key).unwrap();
            let app = binding.as_str();

            match app.parse::<mappings::App>() {
                Ok(app) => {
                    let domain_space = mappings::inspect_app(&app);

                    for domain in &domain_space {
                        if modified_line.starts_with(domain) {
                            let parts: Vec<&str> = modified_line.split(":").collect();
                            modified_line = format!("{}:{}", parts[0], value);
                            break;
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Some of the domains doesn't match a valid value, please check your configuration {:?}", app);
                    std::process::exit(1);
                },
            }
        }

        modified_line
    };

    let modified_contents: String = input_contents
        .lines()
        .map(replace_fn)
        .collect::<Vec<String>>()
        .join("\n");

    return modified_contents;
}

