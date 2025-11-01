use acoustid_api::{AcoustIdApi, response::Response};
use clap::{Args, ValueEnum};
use color_eyre::Result;
use metadata::Metadata;

use crate::{
    Execute,
    fpcalc::{Fpcalc, fpcalc},
};

const DOWNLOAD_DIR: &'static str = "/home/lf/test_music";

#[derive(Args, Debug)]
pub struct YoutubeArgs {
    #[arg(long = "client")]
    pub client: String,
    #[arg(short = 'm', long = "metadata")]
    pub metadata: bool,

    #[arg(value_enum)]
    pub source: Source,

    #[arg(required = true, num_args = 1..)]
    pub url_list: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Source {
    Playlist,
    Tracklist,
}

impl YoutubeArgs {
    fn handle_playlist(&self) -> Result<()> {
        let paths = download_playlist(self.url_list[0].as_ref())?;

        for (i, p) in paths.iter().enumerate() {
            let fpcalc = fpcalc(p.as_str())?;

            let api = AcoustIdApi::new(self.client.to_string());
            let res = api.request(fpcalc.duration, fpcalc.fingerprint).send()?;

            let metadata = match res {
                Response::Ok(res) => Metadata::from_response(res)?,
                Response::Error { error, status: _ } => {
                    return Err(error.into());
                }
            };

            if let Some(mt) = metadata
                && self.metadata
            {
                match mt.apply_to_file(p.into()) {
                    Ok(new_path) => {
                        println!(
                            "Saved {} out of {}. Path: {}",
                            i + 1,
                            paths.len(),
                            new_path.to_string_lossy()
                        );
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            };
        }

        Ok(())
    }

    fn handle_tracklist(&self) -> Result<()> {
        for (i, u) in self.url_list.iter().enumerate() {
            let path = download_track(u)?;

            let fpcalc = fpcalc(path.as_str())?;

            let api = AcoustIdApi::new(self.client.to_string());
            let res = api.request(fpcalc.duration, fpcalc.fingerprint).send()?;

            let metadata = match res {
                Response::Ok(res) => Metadata::from_response(res)?,
                Response::Error { error, status: _ } => {
                    return Err(error.into());
                }
            };

            if let Some(mt) = metadata
                && self.metadata
            {
                match mt.apply_to_file(path.into()) {
                    Ok(new_path) => {
                        println!(
                            "Saved {} out of {}. Path: {}",
                            i + 1,
                            self.url_list.len(),
                            new_path.to_string_lossy()
                        );
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            };
        }

        Ok(())
    }
}

impl Execute for YoutubeArgs {
    fn execute(&self) -> Result<()> {
        match self.source {
            Source::Playlist => self.handle_playlist(),
            Source::Tracklist => self.handle_tracklist(),
        }
    }
}

fn download_track(url: &str) -> Result<String> {
    let output = std::process::Command::new("yt-dlp")
        .args([
            "--print",
            "after_move:\"%(filepath)s\"",
            "-x",
            "-f",
            "bestaudio",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "-P",
            DOWNLOAD_DIR,
            url,
        ])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace("\"", "")
        .to_string())
}

fn download_playlist(url: &str) -> Result<Vec<String>> {
    let output = std::process::Command::new("yt-dlp")
        .args([
            "--print",
            "after_move:\"%(filepath)s\"",
            "-x",
            "-f",
            "bestaudio",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "-P",
            DOWNLOAD_DIR,
            url,
        ])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace("\"", "")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}
