use std::path::Path;

use common::error::*;
use common::profile::*;

use crate::snap_mode::*;
use crate::status::*;

pub const DEFAULT_WORLD_WIDTH: i32 = 3840;
pub const DEFAULT_WORLD_HEIGHT: i32 = 2160;
pub const WORLD_PADDING: i32 = 500;
pub const DESKTOP_BORDER: i32 = 3;
pub const DESKTOP_FOOTER: i32 = 10;

const GRID_SIZES: &[i32] = &[1, 8, 16, 32, 64];

#[derive(Debug, Clone)]
pub struct Guide {
    pub kind: GuideKind,
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum GuideKind {
    Horizontal,
    Vertical,
}

#[derive(Debug, Default)]
struct DragState {
    pub dragging: bool,
    pub monitor_idx: usize,
    pub offset_x: i32,
    pub offset_y: i32,
}

pub struct State {
    profile: Profile,
    previous_profile: Option<Profile>,
    selected: usize,

    world_width: i32,
    world_height: i32,
    term_width: u16,
    term_height: u16,

    snap_mode: SnapMode,
    snap_thresh: i32,
    guides: Vec<Guide>,
    selected_grid: usize,

    status: Status,

    drag_state: Option<DragState>,
}

impl State {
    pub async fn new(term_width: u16, term_height: u16) -> Result<Self, Error> {
        let mut state = Self {
            profile: Profile::new().await?,
            previous_profile: None,
            selected: 0,
            world_width: DEFAULT_WORLD_WIDTH,
            world_height: DEFAULT_WORLD_HEIGHT,
            term_width,
            term_height,
            snap_mode: SnapMode::Edges,
            snap_thresh: 10,
            guides: vec![],
            selected_grid: 3,
            status: Status::LoadingMonitors,
            drag_state: None,
        };

        state.load_monitors().await;

        Ok(state)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Global size
    // ─────────────────────────────────────────────────────────────────────────

    pub fn resize(&mut self, term_width: u16, term_height: u16) {
        self.term_width = term_width;
        self.term_height = term_height;
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Profiles
    // ─────────────────────────────────────────────────────────────────────────

    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = profile;
        self.selected = 0;
        self.previous_profile = None;
        self.update_world();
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Monitors
    // ─────────────────────────────────────────────────────────────────────────

    async fn load_monitors(&mut self) {
        match crate::hyprland::read_monitors().await {
            Ok(monitors) => {
                self.set_status(Status::MonitorsLoaded(monitors.len()));
                self.profile.monitors = monitors;
            }

            Err(e) => self.status = Status::Error(format!("Error loading monitors: {e}")),
        };

        if self.selected >= self.profile.monitors.len() {
            self.selected = 0;
        }

        self.update_world();
    }

    pub fn monitors(&self) -> &[Monitor] {
        &self.profile.monitors
    }

    pub fn monitors_count(&self) -> usize {
        self.profile.monitors.len()
    }

    pub fn monitor_name(&self) -> String {
        self.profile
            .monitors
            .get(self.selected)
            .map(|m| m.name.clone())
            .unwrap_or_default()
    }

    pub fn monitor(&self) -> Option<&Monitor> {
        self.profile.monitors.get(self.selected)
    }

    pub fn monitor_mut(&mut self) -> Option<&mut Monitor> {
        self.profile.monitors.get_mut(self.selected)
    }

    pub fn set_selected(&mut self, idx: usize) {
        self.selected = idx;
    }

    pub fn begin_drag_monitor(&mut self, term_x: i32, term_y: i32) {
        let (world_x, world_y) = self.term_to_world(term_x, term_y);

        if let Some(monitor) = self.profile.monitors.get(self.selected) {
            self.drag_state = Some(DragState {
                dragging: true,
                monitor_idx: self.selected,
                offset_x: world_x - monitor.position.x,
                offset_y: world_y - monitor.position.y,
            });
        }
    }

    pub fn drag_move_monitor(&mut self, x: i32, y: i32) {
        if let Some(drag) = &self.drag_state {
            if !drag.dragging {
                return;
            }

            let (world_x, world_y) = self.term_to_world(x, y);
            let grid_px = self.grid_px();
            let mut new_x = snap_to_grid(world_x - drag.offset_x, grid_px);
            let mut new_y = snap_to_grid(world_y - drag.offset_y, grid_px);

            if self.snap_mode != SnapMode::Off {
                let (snapped_x, snapped_y, guides) =
                    self.compute_snap(drag.monitor_idx, new_x, new_y);

                new_x = snapped_x;
                new_y = snapped_y;
                self.guides = guides;
            }

            if let Some(monitor) = self.profile.monitors.get_mut(drag.monitor_idx) {
                monitor.position.x = new_x;
                monitor.position.y = new_y;
            }
        }
    }

    pub fn end_drag_monitor(&mut self) {
        self.drag_state = None;
        self.guides.clear();
        self.update_world();
    }

    pub fn move_monitor(&mut self, dest_x: i32, dest_y: i32) {
        let grid_px = self.grid_px();

        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            monitor.position.x = snap_to_grid(monitor.position.x + dest_x, grid_px);
            monitor.position.y = snap_to_grid(monitor.position.y + dest_y, grid_px);
        }

        self.update_world();
    }

    pub fn toggle_selected_monitor(&mut self) {
        let active_count = self.profile.monitors.iter().filter(|m| m.active).count();

        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            if monitor.active && active_count <= 1 {
                self.set_status(Status::Error("Cannot disable last active monitor".into()));
                return;
            }

            let name = monitor.name.clone();
            let active = !monitor.active;

            monitor.active = active;
            self.set_status(Status::MonitorState(name, active));
        }
    }

    pub fn select_next_monitor(&mut self) {
        if self.profile.monitors.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.profile.monitors.len();
    }

    pub fn select_previous_monitor(&mut self) {
        if self.profile.monitors.is_empty() {
            return;
        }

        if self.selected == 0 {
            self.selected = self.profile.monitors.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Grid
    // ─────────────────────────────────────────────────────────────────────────

    pub fn grid_px(&self) -> i32 {
        GRID_SIZES[self.selected_grid]
    }

    pub fn cycle_grid(&mut self) {
        self.selected_grid = (self.selected_grid + 1) % GRID_SIZES.len();
        self.set_status(Status::Grid(self.grid_px()));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Status bar
    // ─────────────────────────────────────────────────────────────────────────

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Snapping
    // ─────────────────────────────────────────────────────────────────────────

    pub fn snap_mode(&self) -> SnapMode {
        self.snap_mode
    }

    pub fn cycle_snap(&mut self) {
        self.snap_mode = self.snap_mode.next();
        self.set_status(Status::Snap(self.snap_mode));
    }

    pub fn compute_snap(&self, mon_idx: usize, nx: i32, ny: i32) -> (i32, i32, Vec<Guide>) {
        let mut rx = nx;
        let mut ry = ny;
        let mut guides = vec![];
        let thresh = self.snap_thresh;

        let mon = &self.profile.monitors[mon_idx];
        let mw = mon.effective_width();
        let mh = mon.effective_height();

        for (i, other) in self.profile.monitors.iter().enumerate() {
            if i == mon_idx || !other.active {
                continue;
            }
            let ow = other.effective_width();
            let oh = other.effective_height();

            if matches!(self.snap_mode, SnapMode::Edges | SnapMode::Both) {
                // right edge of other → left edge of mon
                if (nx - (other.position.x + ow)).abs() < thresh {
                    rx = other.position.x + ow;
                    guides.push(Guide {
                        kind: GuideKind::Vertical,
                        value: rx,
                    });
                } else if (nx + mw - other.position.x).abs() < thresh {
                    rx = other.position.x - mw;
                    guides.push(Guide {
                        kind: GuideKind::Vertical,
                        value: other.position.x,
                    });
                } else if (nx - other.position.x).abs() < thresh {
                    rx = other.position.x;
                    guides.push(Guide {
                        kind: GuideKind::Vertical,
                        value: rx,
                    });
                }

                if (ny - (other.position.y + oh)).abs() < thresh {
                    ry = other.position.y + oh;
                    guides.push(Guide {
                        kind: GuideKind::Horizontal,
                        value: ry,
                    });
                } else if (ny + mh - other.position.y).abs() < thresh {
                    ry = other.position.y - mh;
                    guides.push(Guide {
                        kind: GuideKind::Horizontal,
                        value: other.position.y,
                    });
                } else if (ny - other.position.y).abs() < thresh {
                    ry = other.position.y;
                    guides.push(Guide {
                        kind: GuideKind::Horizontal,
                        value: ry,
                    });
                }
            }

            if matches!(self.snap_mode, SnapMode::Centers | SnapMode::Both) {
                let mcx = nx + mw / 2;
                let mcy = ny + mh / 2;
                let ocx = other.position.x + ow / 2;
                let ocy = other.position.y + oh / 2;

                if (mcx - ocx).abs() < thresh {
                    rx = ocx - mw / 2;
                    guides.push(Guide {
                        kind: GuideKind::Vertical,
                        value: ocx,
                    });
                }
                if (mcy - ocy).abs() < thresh {
                    ry = ocy - mh / 2;
                    guides.push(Guide {
                        kind: GuideKind::Horizontal,
                        value: ocy,
                    });
                }
            }
        }

        // Snap to origin
        if nx.abs() < thresh {
            rx = 0;
            guides.push(Guide {
                kind: GuideKind::Vertical,
                value: 0,
            });
        }
        if ny.abs() < thresh {
            ry = 0;
            guides.push(Guide {
                kind: GuideKind::Horizontal,
                value: 0,
            });
        }

        (rx, ry, guides)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Guides
    // ─────────────────────────────────────────────────────────────────────────

    pub fn guides(&self) -> &[Guide] {
        &self.guides
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Resolutions
    // ─────────────────────────────────────────────────────────────────────────

    pub fn monitor_resolutions(&self) -> Vec<Resolution> {
        self.profile
            .monitors
            .get(self.selected)
            .map(|m| m.resolutions.clone())
            .unwrap_or_default()
    }

    pub fn set_monitor_resolution(&mut self, resolution: &Resolution) {
        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            monitor.resolution = *resolution;

            self.update_world();
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Scale
    // ─────────────────────────────────────────────────────────────────────────

    pub fn monitor_scale(&self) -> f64 {
        self.profile
            .monitors
            .get(self.selected)
            .map(|m| m.scale)
            .unwrap_or(1.0)
    }

    pub fn adjust_scale(&mut self, delta: f64) {
        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            let mut scale = (monitor.scale + delta).clamp(0.5, 3.0);
            scale = (scale * 20.0).round() / 20.0; // 0.05 steps

            monitor.scale = scale;
            self.set_status(Status::Scale(scale));
        }

        self.update_world();
    }

    pub fn apply_scale(&mut self, scale: f64) {
        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            monitor.scale = scale;
        }

        self.update_world();
        self.set_status(Status::Scale(scale));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Mirroring
    // ─────────────────────────────────────────────────────────────────────────

    pub fn mirrorable_monitors(&self) -> Vec<&Monitor> {
        let selected_name = self.monitor_name();

        self.profile
            .monitors
            .iter()
            .filter(|m| m.name != selected_name && m.active && !m.is_mirrored)
            .collect()
    }

    pub fn mirror_picker_select_current(&mut self, monitor_name: Option<String>) {
        let selected_name = self.monitor_name();

        // Clear previous mirror relationship
        {
            let old_source = self.profile.monitors.get(self.selected).and_then(|m| {
                if m.is_mirrored {
                    Some(m.mirror_source.clone())
                } else {
                    None
                }
            });

            if let Some(src) = old_source {
                let sel_name = selected_name.clone();
                if let Some(src_mon) = self.profile.monitors.iter_mut().find(|m| m.name == src) {
                    src_mon.mirror_targets.retain(|t| t != &sel_name);
                }
            }
        }

        match monitor_name {
            Some(src_name) => {
                if let Some(m) = self.profile.monitors.get_mut(self.selected) {
                    m.is_mirrored = true;
                    m.mirror_source = src_name.clone();
                }
                let sel_name = selected_name.clone();
                if let Some(src_mon) = self
                    .profile
                    .monitors
                    .iter_mut()
                    .find(|m| m.name == src_name)
                {
                    src_mon.mirror_targets.push(sel_name);
                }
                self.set_status(Status::Mirror(selected_name, Some(src_name)));
            }

            None => {
                if let Some(m) = self.profile.monitors.get_mut(self.selected) {
                    m.is_mirrored = false;
                    m.mirror_source.clear();
                }
                self.set_status(Status::Mirror(selected_name, None));
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Workspaces
    // ─────────────────────────────────────────────────────────────────────────

    pub fn workspaces_as_str(&self) -> String {
        self.profile
            .monitors
            .get(self.selected)
            .map(|m| {
                m.workspaces
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default()
    }

    pub fn apply_workspaces_to_monitor(&mut self, workspaces_ids: Vec<usize>) {
        // Clear these IDs from other monitors
        for (i, monitor) in self.profile.monitors.iter_mut().enumerate() {
            if i != self.selected {
                monitor.workspaces.retain(|id| !workspaces_ids.contains(id));
            }
        }

        if let Some(monitor) = self.profile.monitors.get_mut(self.selected) {
            monitor.workspaces = workspaces_ids.clone();

            let name = monitor.name.clone();

            if workspaces_ids.is_empty() {
                self.set_status(Status::WorkspaceAssignment(name, None));
            } else {
                self.set_status(Status::WorkspaceAssignment(name, Some(workspaces_ids)));
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Hyprland
    // ─────────────────────────────────────────────────────────────────────────

    pub async fn apply_configuration(&self) -> Result<(), Error> {
        self.profile.apply(false, false).await
    }

    pub async fn save_configuration(&self) -> Result<String, Error> {
        let path = self.profile.save().await?;

        let path = path.ok_or(Error::NoFilepath)?.display().to_string();

        Ok(path)
    }

    pub fn save_rollback(&mut self) {
        self.previous_profile = Some(self.profile.clone());
    }

    pub fn revert(&mut self) {
        if let Some(prev) = self.previous_profile.take() {
            self.profile = prev;
            self.update_world();
            self.set_status(Status::ConfigurationReverted);
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Profiles
    // ─────────────────────────────────────────────────────────────────────────

    pub async fn save_to_profile(&mut self, name: String) -> Result<(), Error> {
        let home_dir = std::env::home_dir().ok_or(Error::NoHomeDir)?;

        if !name.is_empty() {
            let filename = format!("{name}.yml");

            let path = Path::new(&home_dir)
                .join(".config")
                .join("hyprmonitors")
                .join("profiles");

            std::fs::create_dir_all(&path)?;

            let path = path.join(filename);

            self.profile.save_to(Path::new(&path)).await?;

            self.set_status(Status::ProfileSaved(name));
        } else {
            self.set_status(Status::Error("Profile name cannot be empty".to_string()));
        }

        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Drawing
    // ─────────────────────────────────────────────────────────────────────────

    fn desktop_width(&self) -> i32 {
        (self.term_width as i32) - DESKTOP_BORDER
    }

    fn desktop_height(&self) -> i32 {
        (self.term_height as i32) - DESKTOP_FOOTER
    }

    pub fn world_to_term(&self, world_x: i32, world_y: i32) -> (i32, i32) {
        let desktop_width = self.desktop_width().max(1);
        let desktop_height = self.desktop_height().max(1);

        let term_x = world_x * desktop_width / self.world_width.max(1);
        let term_y = world_y * desktop_height / self.world_height.max(1);

        (term_x, term_y)
    }

    pub fn term_to_world(&self, term_x: i32, term_y: i32) -> (i32, i32) {
        let desktop_width = self.desktop_width().max(1);
        let desktop_height = self.desktop_height().max(1);

        let world_x = term_x * self.world_width.max(1) / desktop_width;
        let world_y = term_y * self.world_height.max(1) / desktop_height;

        (world_x, world_y)
    }

    pub fn update_world(&mut self) {
        if self.profile.monitors.is_empty() {
            self.world_width = DEFAULT_WORLD_WIDTH;
            self.world_height = DEFAULT_WORLD_HEIGHT;
            return;
        }

        let mut max_x = 0i32;
        let mut max_y = 0i32;

        for m in &self.profile.monitors {
            let rx = m.position.x + m.effective_width();
            let ry = m.position.y + m.effective_height();

            if rx > max_x {
                max_x = rx;
            }

            if ry > max_y {
                max_y = ry;
            }
        }

        self.world_width = max_x + WORLD_PADDING;
        self.world_height = max_y + WORLD_PADDING;
    }

    pub fn hit_test(&self, term_x: i32, term_y: i32) -> Option<usize> {
        let (world_x, world_y) = self.term_to_world(term_x, term_y);

        for (i, m) in self.profile.monitors.iter().enumerate() {
            if world_x >= m.position.x
                && world_x < m.position.x + m.effective_width()
                && world_y >= m.position.y
                && world_y < m.position.y + m.effective_height()
            {
                // Click is on a monitor
                return Some(i);
            }
        }

        None
    }
}

fn snap_to_grid(value: i32, grid: i32) -> i32 {
    if grid <= 1 {
        value
    } else {
        (value / grid) * grid
    }
}
