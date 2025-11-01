use std::fmt::Debug;
use std::fmt::Display;
use std::path::PathBuf;

use acoustid_api::response::OkResponse;
use acoustid_api::response::Recording;
use acoustid_api::response::ReleaseType;
use acoustid_api::response::ResResult;

use color_eyre::Result;
use lofty::config::WriteOptions;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::tag::ItemKey;
use lofty::tag::ItemValue;
use lofty::tag::TagItem;

#[derive(Debug)]
pub enum Error {
    InvalidArtistName,
    InvalidReleaseType,
    InvalidTrackTitle,
    InvalidReleaseTitle,
    InvalidReleaseArtistNames,
    NoPrimaryTag,
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

impl Metadata {
    pub fn file_name(&self) -> String {
        format!(
            "{} - {}.mp3",
            self.artist_names.join(", "),
            self.track_title
        )
    }

    pub fn apply_to_file(self, filepath: PathBuf) -> Result<()> {
        let mut file = lofty::read_from_path(&filepath)?;
        let tags = file.primary_tag_mut().ok_or(Error::NoPrimaryTag)?;
        let parent = filepath.parent().unwrap();
        let mut new_path = PathBuf::new();
        new_path.push(parent);
        new_path.push(format!(
            "{} - {}.mp3",
            self.artist_names.join(", "),
            self.track_title.as_str()
        ));

        tags.insert(TagItem::new(
            ItemKey::TrackArtist,
            ItemValue::Text(self.artist_names.join(", ")),
        ));
        tags.insert(TagItem::new(
            ItemKey::TrackTitle,
            ItemValue::Text(self.track_title),
        ));
        tags.insert(TagItem::new(
            ItemKey::AlbumTitle,
            ItemValue::Text(self.release_title.clone()),
        ));
        tags.insert(TagItem::new(
            ItemKey::AlbumArtist,
            ItemValue::Text(self.release_artists.join(", ")),
        ));

        file.save_to_path(&filepath, WriteOptions::new())?;

        std::fs::rename(&filepath, new_path)?;

        Ok(())
    }

    pub fn from_response(res: OkResponse) -> Result<Option<Metadata>, Error> {
        // If there is no info in database
        if res.results.is_empty() {
            return Ok(None);
        }

        let mut artist_names: Option<Vec<String>> = None;
        let mut release_type: Option<ReleaseType> = None;
        let mut track_title: Option<String> = None;
        let mut release_title: Option<String> = None;
        let mut release_artists: Option<Vec<String>> = None;

        let results = res
            .results
            .into_iter()
            .filter(|r| r.recordings.is_some())
            .map(|mut r| {
                r.recordings = Some(
                    r.recordings
                        .unwrap()
                        .into_iter()
                        .filter(|rec| {
                            rec.artists.is_some()
                                && rec.releasegroups.is_some()
                                && rec.title.is_some()
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
                        .filter(|recroding| recroding.releasegroups.is_some())
                        .collect::<Vec<Recording>>(),
                );
                r
            })
            .collect::<Vec<ResResult>>();

        // If there is no valid recordings after filtering
        if results.is_empty() {
            return Ok(None);
        }

        for res in results.into_iter() {
            for rec in res.recordings.unwrap() {
                if artist_names.is_none() {
                    artist_names = Some(rec.artists.unwrap().into_iter().map(|a| a.name).collect());
                }
                if track_title.is_none() {
                    track_title = Some(rec.title.unwrap());
                }

                for rg in rec.releasegroups.unwrap() {
                    if let Some(rt) = release_type.as_ref() {
                        if matches!(rt, ReleaseType::Album) || matches!(rt, ReleaseType::EP) {
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

        Ok(Some(Metadata {
            artist_names: artist_names.ok_or(Error::InvalidArtistName)?,
            release_type: release_type.ok_or(Error::InvalidReleaseType)?,
            track_title: track_title.ok_or(Error::InvalidTrackTitle)?,
            release_title: release_title.ok_or(Error::InvalidReleaseTitle)?,
            release_artists: release_artists.ok_or(Error::InvalidReleaseArtistNames)?,
        }))
    }
}
