use std::pin::Pin;

use futures::future::FutureExt;
use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

use crate::metadata::providers::{MetadataProvider, QueryExpr};
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

impl MetadataProvider for Client {
    type Response = SearchResponse;
    type Error = super::Error;

    fn run_query<'a>(&self, q: QueryExpr<'a>)
        -> Pin<Box<dyn std::future::Future<Output = Result<Self::Response>> + Send + 'a>>
    {
        let q = Self::build_query(q);
        let url = API_BASE_URL.to_string() + "/recording";
        let res = self.http.get(&url)
            .query(&[("query", q)])
            .send()
            .then(|res| {
                match res {
                    Ok(res) => res.bytes().boxed(),
                    Err(e) => futures::future::err(e).boxed(),
                }
            })
            .map(|res| {
                match res {
                    Ok(bytes) => Ok(serde_json::from_reader(bytes.as_ref())?),
                    Err(e) => Err(e.into()),
                }
            });
        res.boxed()
    }

    // Turns QueryExpr into the (Lucene) search syntax used by MusicBrainz
    fn build_query<'a>(q: QueryExpr<'a>) -> String {
        use QueryExpr::*;
        match q {
            Title(t) => format!("title:{}", t),
            Artist(a) => format!("artist:{}", a),
            Artists(ar) => {
                ar
                    .into_iter()
                    .map(|a| format!("artistname:{}", a))
                    .collect::<Vec<String>>()
                    .join(" AND ")
            }
            Album(a) => format!("release:{}", a),
            Year(y) => format!("date:{}", y),
            TrackNumber(n) => format!("tnum:{}", n),
            TotalTracks(t) => format!("tracks:{}", t),
            Length(l) => format!("dur:{}", l),
            Custom(k, v) => format!("{}:{}", k, v),
            AND(l, r) => format!("({} AND {})", Self::build_query(*l), Self::build_query(*r)),
            OR(l, r) => format!("({} OR {})", Self::build_query(*l), Self::build_query(*r)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_mb_query_expr() {
        let q = QueryExpr::Title("Great Song");
        assert_eq!("title:Great Song", Client::build_query(q));
    }

    #[test]
    fn mb_query_expr_with_and() {
        let q = QueryExpr::Title("Great Song").and(QueryExpr::Artist("Cool Artist"));
        assert_eq!("(title:Great Song AND artist:Cool Artist)", Client::build_query(q));
    }

    #[test]
    fn mb_query_expr_with_or() {
        let q = QueryExpr::Title("Great Song").or(QueryExpr::Artist("Cool Artist"));
        assert_eq!("(title:Great Song OR artist:Cool Artist)", Client::build_query(q));
    }

    #[test]
    fn mb_query_expr_multiple_ands() {
        let q = QueryExpr::Title("Great Song")
            .and(QueryExpr::Artist("Cool Artist").and(QueryExpr::Album("Amazing Album")));
        let exp = "(title:Great Song AND (artist:Cool Artist AND release:Amazing Album))";
        assert_eq!(exp, Client::build_query(q));
    }

    #[test]
    fn mb_query_expr_ands_and_ors() {
        let q = QueryExpr::Title("Great Song").and(QueryExpr::Artist("Cool Artist"))
            .or(QueryExpr::Title("Less Great Song").and(QueryExpr::Artist("Less Cool Artist")));
        let exp = "((title:Great Song AND artist:Cool Artist) OR (title:Less Great Song AND artist:Less Cool Artist))";
        assert_eq!(exp, Client::build_query(q));
    }
}
