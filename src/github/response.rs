use std::error::Error;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use reqwest::blocking::Client;

pub struct ClientContainer;

impl ClientContainer {
    pub fn new_client() -> Client {
        return Client::new();
    }

    pub fn get_response(url: &str, headers: HeaderMap) -> Result<Response, Box<dyn Error>> {
        let response = ClientContainer::new_client().get(url)
            .headers(headers)
            .send()
            .expect("Something went wrong");

        return Ok(response);
    }

    pub fn post_response(url: &str, headers: HeaderMap, body: String) -> Result<Response, Box<dyn Error>> {
        let res = ClientContainer::new_client().post(url)
            .headers(headers)
            .body(body)
            .send()?;

        return Ok(res)
    }

    pub fn put_response(url: &str, headers: HeaderMap, body: String) -> Result<Response, Box<dyn Error>> {
        let res = ClientContainer::new_client().put(url)
            .headers(headers)
            .body(body)
            .send()?;

        return Ok(res)
    }

    pub fn delete_response(url: &str, headers: HeaderMap) -> Result<Response, Box<dyn Error>> {
        let res = ClientContainer::new_client().delete(url)
            .headers(headers)
            .send()?;

        return Ok(res)
    }
}
