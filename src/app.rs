use crate::events::Event;
use crate::models::User;
use crate::ui::{self, Component, Palette};
use std::io::{self, Write};

/// Stores relevant state for the Application
pub struct AppState {
    dirty: bool,
    view_index: usize,
    quitting: bool,
    pub user: User,
}

impl AppState {
    /// Toggle the dirty flag
    /// Example:
    /// ```
    /// let mut state = AppState {
    ///     dirty: true,
    ///     view_index: 0,
    ///     quittinng: false,
    ///     user: user
    /// };
    /// state.toggle_dirty();
    /// assert_eq!(state.dirty(), false);
    /// ```
    pub fn toggle_dirty(&mut self) {
        self.dirty = !self.dirty;
    }

    /// Build an AppState instance by specifying a User
    /// Example:
    /// ```
    /// let user = User {
    ///     id: 0,
    ///     url: "https://gitgator.com/username",
    ///     login: "username",
    /// };
    /// let mut state = AppState::new(user);
    /// assert_eq!(state.user, user);
    /// ```
    pub fn for_user(user: User) -> AppState {
        AppState {
            dirty: true,
            user,
            view_index: 0,
            quitting: false,
        }
    }
}

pub struct App<'a> {
    palette: Palette<'a>,
    state: AppState,
    views: Vec<Box<dyn Component>>,
}

impl<'a> App<'a> {
    pub fn new(palette: Palette<'a>, state: AppState, views: Vec<Box<dyn Component>>) -> App<'a> {
        App {
            palette,
            state,
            views,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.state.quitting
    }

    fn dirty(&self) -> bool {
        self.views[self.state.view_index].dirty()
    }

    pub fn render(&mut self, screen: &mut Write) -> Result<(), io::Error> {
        if self.dirty() {
            debug!("[app  ] Ui is dirty - rendering...");
            write!(screen, "{}{}", self.palette.dual_reset(), ui::clear_all()).unwrap();
            self.views[self.state.view_index].render(screen, &self.palette)?;
            write!(screen, "{}", ui::cursor_hide())?;
            screen.flush()?;
            self.state.toggle_dirty()
        } else {
            debug!("[app  ] Ui is NOT dirty - skipping render");
        }
        Ok(())
    }

    pub fn handle_event(&mut self, event: Event) {
        debug!("[app  ] Handling event: {:?}", event);
        match event {
            Event::Quit => self.state.quitting = true,
            _ => self.views[self.state.view_index].handle_event(event),
        };
        self.state.toggle_dirty();
    }
}
