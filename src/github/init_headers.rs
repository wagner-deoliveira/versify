use std::env;
use dotenv::dotenv;
use reqwest::header::HeaderMap;

pub enum MediaType {
    RAW,
    HTML,
    JSON,
}

fn get_media_type(media_type: MediaType) -> &'static str {
    match media_type {
        MediaType::RAW => "application/vnd.github.raw",
        MediaType::HTML => "application/vnd.github.html",
        MediaType::JSON => "application/vnd.github+json",
    }
}

pub fn init(api_media: MediaType) -> HeaderMap {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
    let token = format!("Bearer {}", github_token);

    let api_type = get_media_type(api_media);

    let mut headers = HeaderMap::new();
    headers.insert("Accept", api_type.parse().unwrap());
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert("User-Agent", "versify".parse().unwrap());

    return headers;
}
