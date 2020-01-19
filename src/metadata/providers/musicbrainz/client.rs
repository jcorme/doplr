use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

use super::entities::Recording;

const API_BASE_URL: &'static str = "http://musicbrainz.org/ws/2";
const DOPLR_VERSION: &'static str = env!("CARGO_PKG_VERSION");

type Result<T> = std::result::Result<T, super::Error>;

pub struct Client {
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub created: String,
    pub count: usize,
    pub offset: usize,
    // Client currently only searches for recordings
    pub recordings: Vec<Recording>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let http = reqwest::ClientBuilder::new()
            .default_headers(Self::default_headers())
            .build()?;

        Ok(Client {
            http
        })
    }

    pub async fn search_recordings(&self, query: &str) -> Result<SearchResponse> {
        let url = API_BASE_URL.to_string() + "/recording";
        let res = self.http.get(&url)
            .query(&[("query", query)])
            .send()
            .await?;
        let buf = res.bytes().await?;
        let res: SearchResponse = serde_json::from_reader(buf.as_ref()).unwrap();
        Ok(res)
    }

    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        // MusicBrainz rejects requests without valid user agent
        let ua = format!("Doplr/{} ( doplr@jcndrop.com )", DOPLR_VERSION);
        headers.insert(header::USER_AGENT, ua.parse().unwrap());
        // Use the JSON API rather than the default XML API
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers
    }
}
