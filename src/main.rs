use clap::Parser;
use color_eyre::Result;
use serde_json::Value;

const DOWNLOAD_DIR: &'static str = "/home/lf/test_music";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'm', long = "metadata")]
    metadata: bool,
    url_list: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for u in args.url_list.iter() {
        download(u)?;
    }

    Ok(())
}

fn download(url: &str) -> Result<()> {
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

    let value: Value = serde_json::from_str(&fpcalc)?;
    println!("{:?}", value["duration"]);

    Ok(())
}
