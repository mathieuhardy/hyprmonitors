mod options;

use clap::Parser;

use common::error::*;
use common::profile::*;

use options::*;

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
