use reqwest::header;
use serde_json::Value;

pub type ApiResult<T> = Result<T, reqwest::Error>;

pub struct Client<'a> {
    api_key: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(api_key: &'a str) -> Client {
        Client {
            api_key: api_key.into(),
        }
    }

    pub fn get(&self, url_end: &'a str) -> ApiResult<Value> {
        let client = reqwest::Client::new();
        let url = format!("https://api.github.com/{}", url_end);

        let token = format!("token {}", self.api_key);
        let mut res = client
            .get(&url)
            .header(header::AUTHORIZATION, token)
            .send()
            .unwrap();
        let ok: Value = res.json().unwrap();
        Ok(ok)
    }
}
