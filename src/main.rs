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
use version_manager::replace_version::replace_version_to_file;
use github::api::{create_new_branch, list_all_branches};
use crate::github::api::{create_pr, delete_branch, close_pr, download_package, get_open_pull_requests, update_file, update_file_in_branch};

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
        Some(EntityType::DeleteBranch(name)) => {
            Ok(delete_branch(&name.branch).expect("Something went wrong"))
        }
        Some(EntityType::UpdateBranch(name)) => {
            if let Some(path) = &name.path {
                return Ok(update_file(&name.message, &name.target_branch, String::from(path)).expect("Something went wrong"));
            }

            let mut domain_list = vec![];
            let mut version_list = vec![];
            if let Some(domains) = &name.domain {
                domain_list = domains.split(",").collect();
            }

            if let Some(versions) = &name.version {
                version_list = versions.split(",").collect();
            }

            if domain_list.len() != version_list.len() {
                Err::<EntityType, Box<dyn Error>>(Box::try_from("Domain_list and version_list have different sizes. \
                        Make sure to enter the same number of domains and versions").unwrap(),
                ).unwrap();
            };

            let version_mapping: HashMap<&str, &str> = domain_list.clone().into_iter().zip(version_list.clone().into_iter()).collect();
            Ok(update_file_in_branch(&name.message, &name.target_branch, version_mapping).await.expect("Something went wrong"))
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
            replace_version_to_file(&packages, version_mapping, output_path)
        }
        Some(EntityType::List(_)) => {
            let list_branches = list_all_branches();
            for branch in list_branches {
                println!("{}", branch);
            }
            Ok(())
        }
        Some(EntityType::ListPr(_)) => {
            let pr_list = get_open_pull_requests();
            for pr in pr_list {
                println!("{}", pr);
            }
            Ok(())
        }
        Some(EntityType::ClosePr(name)) => {
            Ok(close_pr(&name.pr_number).expect("Something went wrong"))
        }
    }
}
