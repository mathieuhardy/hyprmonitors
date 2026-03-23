mod error;
mod hypr;
mod logs;
mod options;
mod profile;

use clap::Parser;

use error::*;
use options::*;
use profile::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Options::parse();

    let profile = if let Some(profile_path) = opts.profile {
        Profile::from_path(&profile_path).await?
    } else {
        Profile::new().await?
    };

    profile.apply().await?;

    Ok(())
}
