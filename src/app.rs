use crate::events::{Command, Event};
use crate::gh;
use crate::gh::{Client, User};
use crate::ui::{self, Component, Palette};
use crate::views::{FullPullRequest, PullRequestList};
use serde_json::value;
use std::collections::VecDeque;
use std::io::{self, Write};

/// Stores relevant state for the Application
pub struct AppState {
    dirty: bool,
    quitting: bool,
    pub user: User,
}

pub struct Stack<T> {
    inner: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) {
        self.inner.push_front(item)
    }

    pub fn new() -> Stack<T> {
        Stack {
            inner: VecDeque::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
}

impl AppState {
    /// Toggle the dirty flag
    /// Example:
    /// ```
    /// let mut state = AppState {
    ///     dirty: true,
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
            quitting: false,
        }
    }
}

pub struct App<'a> {
    client: Client<'a>,
    palette: Palette<'a>,
    state: AppState,
    current_view: Box<dyn Component<'a>>,
    history: Stack<Command>,
}

pub fn get_pulls(
    client: &gh::Client,
    repo: &str,
) -> Result<Vec<gh::PullRequest>, serde_json::error::Error> {
    let url = format!("repos/bisondev/{}/pulls", repo);
    let json = client.get(&url).unwrap();
    debug!("{:#}", json);
    let prs = value::from_value::<Vec<gh::PullRequest>>(json);
    debug!("{:#?}", prs);
    prs
}

impl<'a> App<'a> {
    pub fn new(palette: Palette<'a>, state: AppState, client: Client<'a>) -> App<'a> {
        let prs = get_pulls(&client, "backend").unwrap();
        App {
            client,
            palette,
            state,
            history: Stack::new(),
            current_view: Box::new(PullRequestList::new(prs)),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.state.quitting
    }

    fn dirty(&self) -> bool {
        self.current_view.dirty()
    }

    pub fn render(&mut self, screen: &mut Write) -> Result<(), io::Error> {
        if self.dirty() {
            debug!("[app  ] Ui is dirty - rendering...");
            write!(screen, "{}{}", self.palette.dual_reset(), ui::clear_all()).unwrap();
            self.current_view.render(screen, &self.palette)?;
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

        let command: Option<Command> = self.current_view.select();

        match event {
            Event::Quit => self.state.quitting = true,
            Event::Select => {
                if let Some(command) = command {
                    debug!("[CMD]: {:?}", command);
                    match command {
                        Command::ShowPull(pr) => {
                            let prs = get_pulls(&self.client, "backend").unwrap();
                            self.history.push(Command::ListPulls(prs));
                            self.current_view = Box::new(FullPullRequest::new(pr));
                        }
                        _ => {}
                    }
                }
            }
            Event::Back => {
                if let Some(view) = self.history.pop() {
                    match view {
                        Command::ListPulls(prs) => {
                            self.current_view = Box::new(PullRequestList::new(prs))
                        }
                        _ => {}
                    };
                }
            }
            _ => self.current_view.handle_event(event),
        };
        self.state.toggle_dirty();
    }
}
