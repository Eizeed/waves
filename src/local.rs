use std::path::Path;

use acoustid_api::{AcoustIdApi, response::Response};
use clap::Args;
use color_eyre::Result;
use metadata::Metadata;

use crate::{
    Execute,
    fpcalc::{Fpcalc, fpcalc},
};

#[derive(Args, Debug)]
pub struct LocalArgs {
    #[arg(long = "client")]
    pub client: String,
    pub paths: Vec<String>,
}

impl Execute for LocalArgs {
    fn execute(&self) -> Result<()> {
        for path in self.paths.iter() {
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

            if let Some(mt) = metadata {
                let res = mt.apply_to_file(path.into());
                if res.is_ok() {
                    println!("Done");
                }
            };
        }

        Ok(())
    }
}
