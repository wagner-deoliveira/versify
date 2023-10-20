use reqwest::{Client, header};
use dotenv::dotenv;
use std::env;
pub async fn get_packages(branch: &str) -> reqwest::Result<String> {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
    let token = format!("Bearer {}", github_token);
    let repo_url = format!("https://api.github.com/repos/PerkinElmer/srp-spotfire-addins/contents/packages.txt?ref={branch}");

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "application/vnd.github.raw".parse().unwrap());
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert("User-Agent", "reqwst".parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let packages = client.get(repo_url)
        .headers(headers)
        .send()
        .expect("Something went wrong")
        .text();


    return packages;
}