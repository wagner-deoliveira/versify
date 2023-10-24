use crate::github::init_headers::init;
use crate::github::init_headers::MediaType::RAW;

pub async fn get_packages(branch: &str) -> reqwest::Result<String> {
    let repo_url = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/contents/packages.txt?ref={branch}");
    let headers = init(RAW);

    let client = reqwest::blocking::Client::new();
    let packages = client.get(repo_url)
        .headers(headers)
        .send()
        .expect("Something went wrong")
        .text();

    return packages;
}