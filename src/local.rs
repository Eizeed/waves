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
    #[arg(short = 'm', long = "metadata")]
    pub metadata: bool,
}

impl Execute for LocalArgs {
    fn execute(&self) -> Result<()> {
        for (i, path) in self.paths.iter().enumerate() {
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
                            self.paths.len(),
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
