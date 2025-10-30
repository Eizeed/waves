use std::fmt::Debug;
use std::fmt::Display;

use acoustid_api::response::OkResponse;
use acoustid_api::response::Recording;
use acoustid_api::response::ReleaseType;

#[derive(Debug)]
pub enum Error {
    InvalidArtistName,
    InvalidReleaseType,
    InvalidTrackTitle,
    InvalidReleaseTitle,
    InvalidReleaseArtistNames,
    NoReleaseGroup,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub artist_names: Vec<String>,
    pub release_type: ReleaseType,
    pub track_title: String,
    pub release_title: String,
    pub release_artists: Vec<String>,
}

impl TryFrom<OkResponse> for Metadata {
    type Error = Error;

    fn try_from(value: OkResponse) -> Result<Metadata, Error> {
        let mut artist_names: Option<Vec<String>> = None;
        let mut release_type: Option<ReleaseType> = None;
        let mut track_title: Option<String> = None;
        let mut release_title: Option<String> = None;
        let mut release_artists: Option<Vec<String>> = None;

        let res_recordings = value.results.into_iter().map(|mut r| {
            r.recordings = r
                .recordings
                .into_iter()
                .filter(|rec| {
                    rec.artists.is_some() && rec.releasegroups.is_some() && rec.title.is_some()
                })
                .map(|mut recording| {
                    recording.releasegroups = recording.releasegroups.map(|rg| {
                        rg.into_iter()
                            .filter(|rg| {
                                let r = rg.secondary_types.as_ref().map(|t| {
                                    t.into_iter()
                                        .map(|s| (*s).as_str())
                                        .find(|s| *s == "Remix")
                                        .is_some()
                                });
                                if let Some(r) = r { r } else { true }
                            })
                            .collect()
                    });
                    recording
                })
                .collect::<Vec<Recording>>();
            r
        });

        for res in res_recordings {
            for rec in res.recordings {
                if artist_names.is_none() {
                    artist_names = Some(rec.artists.unwrap().into_iter().map(|a| a.name).collect());
                }
                if track_title.is_none() {
                    track_title = Some(rec.title.unwrap());
                }

                if rec.releasegroups.as_ref().unwrap().is_empty() {
                    return Err(Error::NoReleaseGroup);
                }

                for rg in rec.releasegroups.unwrap() {
                    if let Some(rt) = release_type.as_ref() {
                        if matches!(rt, ReleaseType::Album) || matches!(rt, ReleaseType::Ep) {
                            continue;
                        } else {
                            release_type = Some(rg.release_type);
                            release_title = Some(rg.title);
                            release_artists =
                                Some(rg.artists.into_iter().map(|a| a.name).collect());
                        }
                    } else {
                        release_type = Some(rg.release_type);
                        release_title = Some(rg.title);
                        release_artists = Some(rg.artists.into_iter().map(|a| a.name).collect());
                    }
                }
            }
        }

        Ok(Metadata {
            artist_names: artist_names.ok_or(Error::InvalidArtistName)?,
            release_type: release_type.ok_or(Error::InvalidReleaseType)?,
            track_title: track_title.ok_or(Error::InvalidTrackTitle)?,
            release_title: release_title.ok_or(Error::InvalidReleaseTitle)?,
            release_artists: release_artists.ok_or(Error::InvalidReleaseArtistNames)?,
        })
    }
}
