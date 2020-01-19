use std::fmt;

use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};

#[derive(Debug, Deserialize)]
pub struct Alias {
    pub locale: Option<String>,
    #[serde(rename = "sort-name")]
    pub sort_name: String,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    #[serde(rename = "type-id")]
    pub type_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(rename = "sort-name")]
    pub sort_name: String,
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistCredit {
    pub name: Option<String>,
    pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct Recording {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    pub length: Option<u32>,
    pub releases: Option<Vec<Release>>,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub lc: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub aliases: Vec<String>,
    pub country: String,
}

#[derive(Debug)]
pub enum ReleaseStatus {
    Official,
    Promotional,
    Bootleg,
    PseudoRelease,
    Unknown(String),
}

impl<'de> Deserialize<'de> for ReleaseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct ReleaseStatusVisitor;

        impl<'de> Visitor<'de> for ReleaseStatusVisitor {
            type Value = ReleaseStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`Official`, `Promotional`, `Bootleg`, or `Pseudo-Release`")
            }

            fn visit_str<E>(self, value: &str) -> Result<ReleaseStatus, E>
            where
                E: de::Error,
            {
                match value {
                    "Official" => Ok(ReleaseStatus::Official),
                    "Promotional" => Ok(ReleaseStatus::Promotional),
                    "Bootleg" => Ok(ReleaseStatus::Bootleg),
                    "Pseudo-Release" => Ok(ReleaseStatus::PseudoRelease),
                    s => Ok(ReleaseStatus::Unknown(s.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ReleaseStatusVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub id: String,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub date: Option<String>,
    pub country: Option<String>,
    pub status: ReleaseStatus,
    pub media: Vec<Medium>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseGroup {
    pub id: String,
    pub title: String,
    #[serde(rename = "type-id")]
    pub type_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: String,
    pub number: String,
    pub title: String,
    pub length: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Medium {
    pub position: u16,
    pub format: Option<String>,
    pub track: Vec<Track>,
}
