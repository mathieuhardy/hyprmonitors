mod error;
mod hypr;
mod logs;

use error::*;
use hypr::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let workspace_id = active_workspace_id().await?;
    let internal_monitor_name = internal_monitor_name().await?;
    let external_monitors_names = external_monitors_names().await?;
    let is_lid_open = is_lid_open().await?;

    match (
        internal_monitor_name.clone(),
        is_lid_open,
        external_monitors_names.first(),
    ) {
        // - Internal monitor found
        // - Lid opened
        // - No external found
        (Some(internal_monitor), true, None) => {
            // Assign all workspaces to internal
            for i in 1..=10 {
                assign_workspace(i, &internal_monitor, i == 1).await?;
            }

            for i in 1..=10 {
                move_workspace_to_monitor(i, &internal_monitor).await?;
            }

            // Enable monitor
            enable_monitor(&internal_monitor).await?;

            // Move workspace if needed
            for i in 1..=10 {
                move_workspace_to_monitor(i, &internal_monitor).await?;
            }
        }

        // - Lid closed
        // - External found
        (_, false, Some(external_monitor)) => {
            if let Some(internal_monitor) = internal_monitor_name {
                // Disable internal monitor
                disable_monitor(&internal_monitor).await?;
            }

            // Assign all workspaces to external
            for i in 1..=10 {
                assign_workspace(i, external_monitor, i == 1).await?;
            }

            for i in 1..=10 {
                move_workspace_to_monitor(i, external_monitor).await?;
            }
        }

        // - Internal monitor found
        // - Lid opened
        // - External found
        (Some(internal_monitor), true, Some(external_monitor)) => {
            // All except 10 to external
            for i in 1..=9 {
                assign_workspace(i, external_monitor, i == 1).await?;
            }

            assign_workspace(10, &internal_monitor, true).await?;

            // Enable monitor
            enable_monitor(&internal_monitor).await?;

            // Move workspaces
            for i in 1..=9 {
                move_workspace_to_monitor(i, external_monitor).await?;
            }

            move_workspace_to_monitor(10, &internal_monitor).await?;
        }

        _ => (),
    }

    if let Some(workspace_id) = workspace_id {
        jump_to_workspace(workspace_id).await?;
    }

    Ok(())
}
