use common::error::*;
use common::hyprland::*;
use common::profile::*;

pub async fn read_monitors() -> Result<Vec<Monitor>, Error> {
    let hypr_monitors = HyprMonitor::get_all().await?;

    let mut monitors: Vec<Monitor> = hypr_monitors.into_iter().map(From::from).collect();

    // Build mirror targets
    let sources: Vec<(usize, String)> = monitors
        .iter()
        .enumerate()
        .filter(|(_, monitor)| monitor.is_mirrored)
        .map(|(idx, monitor)| (idx, monitor.mirror_source.clone()))
        .collect();

    for (mirror_idx, source_name) in sources {
        let mirror_name = monitors[mirror_idx].name.clone();
        if let Some(src) = monitors.iter_mut().find(|m| m.name == source_name) {
            src.mirror_targets.push(mirror_name);
        }
    }

    Ok(monitors)
}
