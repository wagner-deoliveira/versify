mod args;
mod mappings;
mod read_file;
mod replace_version;

use std::collections::HashMap;
use std::error::Error;
use clap::Parser;
use args::VersifyArgs;
use read_file::read_file;
use replace_version::replace_version;

fn main() -> Result<(), Box<dyn Error>> {
    let args = VersifyArgs::parse();
    let path = args.path;
    let domains = args.domain;
    let versions = args.build_number;
    let mut output_path= "output";

    if let Some(output) = args.output.as_deref() {
        output_path = output;
    }


    let domain_list: Vec<&str> = domains.split(",").collect();
    let version_list: Vec<&str> = versions.split(",").collect();

    if domain_list.len() != version_list.len() {
        return Err(
            Box::try_from("Domain_list and version_list have different sizes. \
            Make sure to enter the same number of domains and versions").unwrap(),
        );
    };

    let version_mapping: HashMap<&str, &str> = domain_list.clone().into_iter().zip(version_list.clone().into_iter()).collect();

    let file = read_file(&*path);
    replace_version(&file, version_mapping, output_path)
}
