use color_eyre::Result;

use crate::response::Response;

pub mod response;

const ACOUSTID_API_URL: &'static str = "https://api.acoustid.org/v2/lookup";

pub struct AcoustIdApi {
    client_api_key: String,
}

pub struct Request {
    url: String,
}

impl AcoustIdApi {
    pub fn new(client_api_key: String) -> Self {
        AcoustIdApi { client_api_key }
    }

    pub fn request(&self, duration_in_secs: u64, fingerprint: String) -> Request {
        let url = format!(
            "{}?meta=recordings+releasegroups&client={}&duration={}&fingerprint={}",
            ACOUSTID_API_URL, self.client_api_key, duration_in_secs, fingerprint
        );
        Request { url }
    }
}

impl Request {
    pub fn send(self) -> Result<Response> {
        // println!("{}", self.url);
        reqwest::blocking::get(self.url)?
            .json()
            .map_err(|e| e.into())
    }
}
