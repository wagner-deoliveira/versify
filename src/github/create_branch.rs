use std::error::Error;
use serde::Deserialize;
use crate::github::init_headers::{init, MediaType};
use base64::{Engine as _, engine::general_purpose};

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

    let body_post = format!("{{\"ref\": \"refs/heads/{}\", \"sha\": \"{}\"}}", branch_name, get_branch.object.sha);
    client.post("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/git/refs")
        .headers(headers)
        .body(body_post)
        .send()?
        .text()?;

    Ok(())
}

pub fn list_all_branches() -> Result<(), Box<dyn Error>>{
    let repo_branch_list = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/branches");

    let headers = init(JSON);

    let client = reqwest::blocking::Client::new();
    let res = client.get(repo_branch_list)
        .headers(headers)
        .send()
        .expect("Something went wrong");

    let branches = res.json::<Branches>().expect("Maybe something went wrong");

    for branch in &branches {
        println!("{:?}", branch.name);
    }

    Ok(())
}
fn update_file_in_branch(message: &str, target_branch: &str, content: String) -> Result<(), Box<dyn Error>> {
    let headers = init(JSON);
    let content_url = format!("https://api.github.com/repos/perkinelmer/srp-spotfire-addins/contents/packages.txt?ref={}", target_branch);

    let client = reqwest::blocking::Client::new();
    let res = client.put(content_url)
        .headers(headers)
        .body("{\"message\":\"my commit message\",\"committer\":{\"name\":\"Monalisa Octocat\",\"email\":\"octocat@github.com\"},\"content\":\"bXkgbmV3IGZpbGUgY29udGVudHM=\"}")
        .send()?;

    Ok(())
}



