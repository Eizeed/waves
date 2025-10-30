use clap::Parser;
use color_eyre::Result;

use crate::youtube::YoutubeArgs;

mod youtube;

#[derive(Parser, Debug)]
enum Args {
    Youtube(YoutubeArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::Youtube(args) => args.execute()?,
    }

    Ok(())
}

pub trait Execute {
    fn execute(&self) -> Result<()>;
}
