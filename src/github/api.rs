use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde::Deserialize;
use crate::github::init_headers::{init, MediaType};
use crate::reader::read_file::read_file;
use base64::{engine::general_purpose, Engine};
use crate::github::get_packages_from_github::get_packages;
use crate::github::response::ClientContainer;
use crate::version_manager::replace_version::replace_version_in_repository;

type References = Vec<RefHead>;
type Branches = Vec<Branch>;
type PullList = Vec<PullRequest>;

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct PullRequest {
    html_url: String,
    title: String,
    user: User,
    state: String,
    body: String,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct User {
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
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

    let refs_heads = ClientContainer::get_response(repo_branch_list.as_str(), headers)?.json::<References>().unwrap();

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
    let url = "https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/git/refs";

    ClientContainer::post_response(url, headers, body_post)?.text()?;

    Ok(println!("Branch created: {}", &branch_name))
}

pub fn list_all_branches() -> Vec<String> {
    let repo_branch_list = "https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/branches";
    let headers = init(JSON);

    let branches = ClientContainer::get_response(repo_branch_list, headers).expect("Maybe something went wrong").json::<Branches>().unwrap();
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

    let res = ClientContainer::post_response(pull_url, headers, body);

    Ok(println!("Status code: {:?}", res?.status()))
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

    let content = ClientContainer::get_response(&content_url, headers.clone())?.json::<Content>().unwrap();
    let file = read_file(&path_to_content);
    let encoded_content = general_purpose::STANDARD.encode(file);
    let body = format!("{{\"message\":\"{}\",\"committer\":{{\"name\":\"Wagner Rosa\",\"email\":\"wagner.deoliveira@revvity.com\"}},\"content\":\"{}\",\"sha\": \"{}\",\"branch\":\"{}\" }}", message, encoded_content, content.sha, target_branch);

    let res = ClientContainer::put_response(
        content_url.as_str(),
        headers,
        body,
    );

    Ok(println!("Status code: {}", res?.status()))
}

pub async fn update_file_in_branch(message: &str, target_branch: &str, map: HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
    let content_url = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/contents/packages.txt?ref={}", target_branch);
    let headers = init(JSON);

    let content = ClientContainer::get_response(&content_url, headers.clone())?.json::<Content>().unwrap();

    let file = get_packages(&target_branch).await.expect("Failed to download package");
    let content_from_branch = replace_version_in_repository(file.as_str(), map);
    let encoded_content = general_purpose::STANDARD.encode(content_from_branch);
    let body = format!("{{\"message\":\"{}\",\"committer\":{{\"name\":\"Wagner Rosa\",\"email\":\"wagner.deoliveira@revvity.com\"}},\"content\":\"{}\",\"sha\": \"{}\",\"branch\":\"{}\" }}", message, encoded_content, content.sha, target_branch);

    let res = ClientContainer::put_response(content_url.as_str(), headers, body);

    Ok(println!("Status code: {}", res?.status()))
}

pub fn get_open_pull_requests() -> Vec<String> {
    let pull_url = "https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/pulls";
    let headers = init(JSON);

    let list_of_pulls_requests = ClientContainer::get_response(pull_url, headers).expect("Something wrong happened").json::<PullList>().unwrap();
    let mut pull_requests_info = Vec::new();

    if list_of_pulls_requests.is_empty() {
        pull_requests_info.push(format!("There are no pull requests open at this moment at {}", pull_url));
        return pull_requests_info;
    }

    let spacer = "-".repeat(70);
    for pull_request in list_of_pulls_requests.iter() {
        pull_requests_info.push(format!("Pull request title: {:?}\nAuthor: {:?}\nurl: {:?}\nState: {:?}\nCreated at: {:?}\nUpdated at: {:?}\n{}\n", pull_request.title, pull_request.user.login, pull_request.html_url, pull_request.state, pull_request.created_at, pull_request.updated_at, spacer))
    }

    return pull_requests_info;
}