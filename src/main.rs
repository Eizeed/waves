use clap::Parser;
use color_eyre::Result;

use crate::{local::LocalArgs, youtube::YoutubeArgs};

mod fpcalc;
mod local;
mod youtube;

#[derive(Parser, Debug)]
enum Args {
    Youtube(YoutubeArgs),
    Local(LocalArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::Youtube(args) => args.execute()?,
        Args::Local(args) => args.execute()?,
    }

    Ok(())
}

pub trait Execute {
    fn execute(&self) -> Result<()>;
}
