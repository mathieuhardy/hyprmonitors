use crossterm::event::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::mouse::*;
use crate::state::*;
use crate::views::*;

pub struct UserInterface {
    active_view: ActiveView,
    advanced_settings_view: advanced_settings::AdvanceSettingsView,
    help_view: help::HelpView,
    main_view: main::MainView,
    mirror_picker_view: mirror_picker::MirrorPickerView,
    resolution_picker_view: resolution_picker::ResolutionPickerView,
    profile_input_view: profile_input::ProfileInputView,
    scale_picker_view: scale_picker::ScalePickerView,
    workspace_assign_view: workspace_assign::WorkspaceAssignView,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::Main,
            advanced_settings_view: advanced_settings::AdvanceSettingsView::new(),
            help_view: help::HelpView::new(),
            main_view: main::MainView::new(),
            mirror_picker_view: mirror_picker::MirrorPickerView::new(),
            resolution_picker_view: resolution_picker::ResolutionPickerView::new(),
            profile_input_view: profile_input::ProfileInputView::new(),
            scale_picker_view: scale_picker::ScalePickerView::new(),
            workspace_assign_view: workspace_assign::WorkspaceAssignView::new(),
        }
    }

    pub fn set_active_view(&mut self, view: ActiveView) {
        self.active_view = view;
    }

    pub fn reset(&mut self, state: &State) {
        match self.active_view {
            ActiveView::AdvancedSettings => self.advanced_settings_view.reset(state),
            ActiveView::Help => self.help_view.reset(state),
            ActiveView::Main => self.main_view.reset(state),
            ActiveView::MirrorPicker => self.mirror_picker_view.reset(state),
            ActiveView::ProfileInput => self.profile_input_view.reset(state),
            ActiveView::ResolutionPicker => self.resolution_picker_view.reset(state),
            ActiveView::ScalePicker => self.scale_picker_view.reset(state),
            ActiveView::WorkspaceAssign => self.workspace_assign_view.reset(state),
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &mut State) {
        match self.active_view {
            ActiveView::AdvancedSettings => self.advanced_settings_view.render(frame, state),
            ActiveView::Help => self.help_view.render(frame, state),
            ActiveView::Main => self.main_view.render(frame, state),
            ActiveView::MirrorPicker => self.mirror_picker_view.render(frame, state),
            ActiveView::ProfileInput => self.profile_input_view.render(frame, state),
            ActiveView::ResolutionPicker => self.resolution_picker_view.render(frame, state),
            ActiveView::ScalePicker => self.scale_picker_view.render(frame, state),
            ActiveView::WorkspaceAssign => self.workspace_assign_view.render(frame, state),
        }
    }

    pub async fn handle_event(&mut self, state: &mut State, event: Event) -> Result<Action, Error> {
        match event {
            Event::Key(key) => self.handle_key(key, state).await,

            Event::Mouse(mouse) if self.active_view == ActiveView::Main => {
                handle_mouse(state, mouse);
                Ok(Action::None)
            }

            Event::Resize(width, height) => {
                state.resize(width, height);
                Ok(Action::None)
            }

            _ => Ok(Action::None),
        }
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Ok(Action::Quit);
        }

        match self.active_view {
            ActiveView::AdvancedSettings => {
                self.advanced_settings_view.handle_key(key, state).await
            }
            ActiveView::Help => self.help_view.handle_key(key, state).await,
            ActiveView::Main => self.main_view.handle_key(key, state).await,
            ActiveView::MirrorPicker => self.mirror_picker_view.handle_key(key, state).await,
            ActiveView::ProfileInput => self.profile_input_view.handle_key(key, state).await,
            ActiveView::ResolutionPicker => {
                self.resolution_picker_view.handle_key(key, state).await
            }
            ActiveView::ScalePicker => self.scale_picker_view.handle_key(key, state).await,
            ActiveView::WorkspaceAssign => self.workspace_assign_view.handle_key(key, state).await,
        }
    }
}
