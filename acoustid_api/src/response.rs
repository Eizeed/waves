use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Ok {
        results: Vec<ResResult>,
        status: Status,
    },
    Error {
        error: Error,
        status: Status,
    },
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
    duraiton: Option<f32>,
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
    title: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
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
