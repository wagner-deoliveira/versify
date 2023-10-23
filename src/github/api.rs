use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde::Deserialize;
use crate::github::init_headers::{init, MediaType};
use crate::reader::read_file::read_file;
use base64::{engine::general_purpose, Engine};
use crate::github::get_packages_from_github::get_packages;
use crate::version_manager::replace_version::replace_version_in_repository;

type References = Vec<RefHead>;
type Branches = Vec<Branch>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct Object {
    sha: String,
    #[serde(rename = "type")]
    _type: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct Commit {
    sha: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct RefHead {
    #[serde(rename = "ref")]
    ref_name: String,
    node_id: String,
    url: String,
    object: Object,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct Branch {
    name: String,
    commit: Commit,
    protected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct Content {
    name: String,
    path: String,
    sha: String,
    size: u16,
    url: String,
    html_url: String,
    git_url: String,
    download_url: String,
    #[serde(rename = "type")]
    _type: String,
    content: String,
    encoding: String,
    _links: HashMap<String, String>,
}

trait Contains {
    fn contains(&self, param: &str) -> String;
}

impl Contains for RefHead {
    fn contains(&self, param: &str) -> String {
        return self.ref_name.contains(&param).to_string();
    }
}

const RAW: MediaType = MediaType::RAW;
const JSON: MediaType = MediaType::JSON;

pub fn create_new_branch(branch_source: &str, branch_name: &str) -> Result<(), Box<dyn Error>> {
    let repo_branch_list = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/git/refs/heads");
    let mut headers = init(RAW);

    let client = reqwest::blocking::Client::new();
    let res = client.get(repo_branch_list)
        .headers(headers)
        .send()
        .expect("Something went wrong");

    let refs_heads = res.json::<References>().unwrap();

    let target_value = format!("refs/heads/{}", branch_source);
    let mut get_branch = RefHead::default();
    for heads in &refs_heads {
        if heads.ref_name.eq(&target_value) {
            get_branch = heads.to_owned();
        }
    }

    headers = init(JSON);

    if get_branch.object.sha.is_empty() {
        return Err::<Result<(), Box<(dyn Error + 'static)>>, Box<dyn Error>>(Box::try_from("No branch has been found with this conditions").unwrap()).unwrap();
    }

    let body_post = format!("{{\"ref\": \"refs/heads/{}\",\"sha\": \"{}\"}}", branch_name, get_branch.object.sha);
    client.post("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/git/refs")
        .headers(headers)
        .body(body_post)
        .send()?
        .text()?;

    Ok(println!("Branch creaated: {}", &branch_name))
}

pub fn list_all_branches() -> Vec<String> {
    let repo_branch_list = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/branches");

    let headers = init(JSON);

    let client = reqwest::blocking::Client::new();
    let res = client.get(repo_branch_list)
        .headers(headers)
        .send()
        .expect("Something went wrong");

    let branches = res.json::<Branches>().expect("Maybe something went wrong");
    let mut branch_names: Vec<String> = vec![];

    for branch in &branches {
        branch_names.push(String::from(&branch.name));
    }

    return branch_names;
}

pub fn create_pr(title: &str, body: &str, branch: &str, target_branch: &str) -> Result<(), Box<dyn Error>> {
    let headers = init(JSON);
    let pull_url = "https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/pulls";

    let body = format!("{{\"title\":\"{}\",\"body\":\"{}\",\"head\":\"{}\", \"base\": \"{}\"}}", title, body, branch, target_branch);

    let client = reqwest::blocking::Client::new();
    let res = client.post(pull_url)
        .headers(headers)
        .body(body)
        .send()?;


    Ok(println!("Status code: {:?}", res.status()))
}

pub async fn download_package(branch: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let file = get_packages(branch).await.expect("Failed to download package");
    fs::create_dir_all(output_path)?;

    let output_file_path = format!("{}/packages.txt", &output_path);
    fs::write(output_file_path, file)?;

    Ok(())
}

pub fn update_file(message: &str, target_branch: &str, path_to_content: String) -> Result<(), Box<dyn Error>> {
    let content_url = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/contents/packages.txt?ref={}", target_branch);
    let headers = init(JSON);

    let client = reqwest::blocking::Client::new();
    let res = client.get(&content_url)
        .headers(headers.clone())
        .send()
        .expect("Something went wrong");

    let content = res.json::<Content>().unwrap();
    let file = read_file(&path_to_content);
    let encoded_content = general_purpose::STANDARD.encode(file);
    let body = format!("{{\"message\":\"{}\",\"committer\":{{\"name\":\"Wagner Rosa\",\"email\":\"wagner.deoliveira@revvity.com\"}},\"content\":\"{}\",\"sha\": \"{}\",\"branch\":\"{}\" }}", message, encoded_content, content.sha, target_branch);

    let client = reqwest::blocking::Client::new();
    let res = client.put(content_url)
        .headers(headers)
        .body(body)
        .send()?;

    Ok(println!("Status code: {}", res.status()))
}

pub async fn update_file_in_branch(message: &str, target_branch: &str, map: HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
    let content_url = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/contents/packages.txt?ref={}", target_branch);
    let headers = init(JSON);

    let client = reqwest::blocking::Client::new();
    let res = client.get(&content_url)
        .headers(headers.clone())
        .send()
        .expect("Something went wrong");

    let content = res.json::<Content>().unwrap();

    let file = get_packages(&target_branch).await.expect("Failed to download package");
    let content_from_branch = replace_version_in_repository(file.as_str(), map);
    let encoded_content = general_purpose::STANDARD.encode(content_from_branch);
    let body = format!("{{\"message\":\"{}\",\"committer\":{{\"name\":\"Wagner Rosa\",\"email\":\"wagner.deoliveira@revvity.com\"}},\"content\":\"{}\",\"sha\": \"{}\",\"branch\":\"{}\" }}", message, encoded_content, content.sha, target_branch);

    let client = reqwest::blocking::Client::new();
    let res = client.put(content_url)
        .headers(headers)
        .body(body)
        .send()?;

    Ok(println!("Status code: {}", res.status()))
}

