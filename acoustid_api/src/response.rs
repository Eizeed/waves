use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Ok(OkResponse),
    Error { error: Error, status: Status },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OkResponse {
    pub results: Vec<ResResult>,
    pub status: Status,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResResult {
    pub id: Uuid,
    pub recordings: Option<Vec<Recording>>,
    pub score: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Recording {
    pub id: Uuid,
    pub artists: Option<Vec<Artist>>,
    pub duration: Option<f32>,
    pub releasegroups: Option<Vec<ReleaseGroup>>,
    pub title: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseGroup {
    pub id: Uuid,
    pub artists: Vec<Artist>,
    #[serde(alias = "secondarytypes")]
    pub secondary_types: Option<Vec<String>>,
    pub title: String,
    #[serde(alias = "type")]
    pub release_type: ReleaseType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum ReleaseType {
    Single,
    Album,
    EP,
    Other,
}
