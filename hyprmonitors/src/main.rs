mod options;

use clap::Parser;

use common::error::*;
use common::hyprland::*;
use common::profile::*;

use options::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Options::parse();
    let is_verbose = opts.verbose > 0;

    match opts.profile {
        Some(path) => {
            let profile = Profile::from_path(&path).await?;
            profile.apply(is_verbose).await?;
        }

        None => {
            let profiles = load_profiles().await?;
            let current_monitors: Vec<Monitor> = HyprMonitor::get_all()
                .await?
                .into_iter()
                .map(From::from)
                .collect();

            let mut profiles_ranked: Vec<_> = profiles
                .iter()
                .map(|profile| {
                    let mut score = 0;

                    for monitor in &current_monitors {
                        if profile.monitors.iter().any(|m| m.name == monitor.name) {
                            score += 1;
                        }
                    }

                    (score, profile)
                })
                .collect();

            profiles_ranked.sort_by(|a, b| b.0.cmp(&a.0));

            if let Some((_, profile)) = profiles_ranked.last() {
                profile.apply(is_verbose).await?;
            }
        }
    }

    Ok(())
}
