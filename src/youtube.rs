use acoustid_api::{AcoustIdApi, response::Response};
use clap::Args;
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
    pub url_list: Vec<String>,
    #[arg(short = 'm', long = "metadata")]
    pub metadata: bool,
}

impl Execute for YoutubeArgs {
    fn execute(&self) -> Result<()> {
        for (i, u) in self.url_list.iter().enumerate() {
            let path = download(u)?;

            let Fpcalc {
                duration,
                fingerprint,
                ..
            } = fpcalc(path.as_str())?;

            let api = AcoustIdApi::new(self.client.to_string());
            let res = api.request(duration, fingerprint).send()?;

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

fn download(url: &str) -> Result<String> {
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
