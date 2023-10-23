mod args;
mod reader;
mod github;
mod version_manager;

use std::collections::HashMap;
use std::error::Error;
use clap::Parser;
use args::VersifyArgs;
use args::EntityType;
use reader::read_file::read_file;
use version_manager::replace_version::replace_version;
use github::api::{create_new_branch, list_all_branches};
use crate::github::api::{create_pr, download_package, update_file_in_branch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = VersifyArgs::parse();

    match &args.command {
        None => {
            Err(Box::try_from("Domain_list and version_list have different sizes. \
                        Make sure to enter the same number of domains and versions").unwrap(),
            )
        }
        Some(EntityType::CreateBranch(name)) => {
            Ok(create_new_branch(&name.source, &name.new_branch).expect("Something went wrong"))
        }
        Some(EntityType::UpdateBranch(name)) => {
            Ok(update_file_in_branch(&name.message, &name.target_branch, String::from(&name.path)).expect("Something went wrong"))
        }
        Some(EntityType::CreatePr(name)) => {
            Ok(create_pr(&name.title, &name.message, &name.branch, &name.target_branch).expect("Something went wrong"))
        }
        Some(EntityType::Download(name)) => {
            let mut output_path = "tmp";
            if let Some(output) = &name.output.as_deref() {
                output_path = output;
            }
            download_package(&name.branch, output_path).await
        }
        Some(EntityType::Update(name)) => {
            let domains = &name.domain;
            let versions = &name.version;
            let mut output_path = "output";
            let mut branch = "main";

            if let Some(branch_name) = &name.branch.as_deref() {
                branch = branch_name
            }
            let mut packages = github::get_packages_from_github::get_packages(branch).await.expect("Something went wrong");

            if let Some(path) = &name.path.as_deref() {
                packages = read_file(&*path);
            }

            if let Some(output) = &name.output.as_deref() {
                output_path = output;
            }
            if let Some(output) = &name.output.as_deref() {
                output_path = output;
            }

            let domain_list: Vec<&str> = domains.split(",").collect();
            let version_list: Vec<&str> = versions.split(",").collect();

            if domain_list.len() != version_list.len() {
                Err::<EntityType, Box<dyn Error>>(Box::try_from("Domain_list and version_list have different sizes. \
                        Make sure to enter the same number of domains and versions").unwrap(),
                ).unwrap();
            };

            let version_mapping: HashMap<&str, &str> = domain_list.clone().into_iter().zip(version_list.clone().into_iter()).collect();
            replace_version(&packages, version_mapping, output_path)
        }
        Some(EntityType::List(_)) => {
            let list_branches = list_all_branches();
            for branch in list_branches {
                println!("{}", branch);
            }
            Ok(())
        }
    }
}
