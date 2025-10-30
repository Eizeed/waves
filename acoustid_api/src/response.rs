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
    id: Uuid,
    recordings: Vec<Recording>,
    score: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Recording {
    id: Uuid,
    artists: Option<Vec<Artist>>,
    duration: Option<f32>,
    releasegroups: Option<Vec<ReleaseGroup>>,
    title: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    id: Uuid,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseGroup {
    id: Uuid,
    artists: Vec<Artist>,
    #[serde(alias = "secondarytypes")]
    secondary_types: Option<Vec<String>>,
    title: String,
    #[serde(alias = "type")]
    release_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    code: i32,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ReleaseType {
    Single,
    Album,
    Ep,
    Other,
}
