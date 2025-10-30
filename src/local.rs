use acoustid_api::{AcoustIdApi, response::Response};
use clap::Args;
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
    fn execute(&self) -> color_eyre::eyre::Result<()> {
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

            println!("{:#?}", metadata);
        }

        Ok(())
    }
}
