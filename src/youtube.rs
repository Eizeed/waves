use acoustid_api::{AcoustIdApi, response::Response};
use clap::Args;
use color_eyre::Result;
use metadata::Metadata;
use serde_json::Value;

use crate::Execute;

const DOWNLOAD_DIR: &'static str = "/home/lf/test_music";

#[derive(Args, Debug)]
pub struct YoutubeArgs {
    #[arg(long = "client")]
    pub client: String,
    #[arg(short = 'm', long = "metadata")]
    pub metadata: bool,
    pub url_list: Vec<String>,
}

impl Execute for YoutubeArgs {
    fn execute(&self) -> Result<()> {
        for u in self.url_list.iter() {
            download(u, self.client.as_str())?;
        }

        Ok(())
    }
}

fn download(url: &str, key: &str) -> Result<()> {
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

    let file_path = String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace("\"", "")
        .to_string();

    let output = std::process::Command::new("fpcalc")
        .arg(&file_path)
        .arg("-json")
        .output()?;

    let fpcalc = String::from_utf8_lossy(&output.stdout);

    let mut value: Value = serde_json::from_str(&fpcalc)?;
    let str = value["fingerprint"].take().to_string().replace("\"", "");
    let dur = value["duration"].as_f64().unwrap().floor() as u64;

    let api = AcoustIdApi::new(key.to_string());
    let res = api.request(dur, str).send()?;

    let metadata = match res {
        Response::Ok(res) => Metadata::try_from(res)?,
        Response::Error { error, status: _ } => {
            return Err(error.into());
        }
    };

    println!("{:#?}", metadata);

    Ok(())
}
