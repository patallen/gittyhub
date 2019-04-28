use super::events::{Direction, Event};
use super::models::{PullRequest, User};
use super::ui;
use super::ui::Palette;
use std::io;
use std::io::Write;
use termion::color::{Bg, Fg};

pub struct AppState {
    dirty: bool,
    view_index: usize,
    quitting: bool,
    pub user: User,
}

impl AppState {
    pub fn toggle_dirty(&mut self) {
        self.dirty = !self.dirty;
    }

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

pub trait Component {
    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error>;
    fn handle_event(&mut self, event: Event);
    fn dirty(&self) -> bool;
}

pub struct PullRequestList<'a> {
    dirty: bool,
    items: Vec<PullRequest<'a>>,
    hovered_index: Option<usize>,
}

impl<'a> PullRequestList<'a> {
    pub fn new(pull_requests: Vec<PullRequest<'a>>) -> PullRequestList<'a> {
        PullRequestList {
            dirty: true,
            items: pull_requests,
            hovered_index: None,
        }
    }

    pub fn hover_down(&mut self) {
        match self.hovered_index {
            None => self.hovered_index = Some(0),
            Some(idx) => {
                if idx < self.items.len() - 1 {
                    self.hovered_index = Some(idx + 1);
                } else {
                    self.hovered_index = Some(0);
                }
            }
        }

        self.dirty = true;
    }

    pub fn hover_up(&mut self) {
        match self.hovered_index {
            None => self.hovered_index = Some(self.items.len() - 1),
            Some(idx) => {
                if idx <= 0 {
                    self.hovered_index = Some(self.items.len() - 1);
                } else {
                    self.hovered_index = Some(idx - 1);
                }
            }
        }

        self.dirty = true;
    }

    pub fn request_selection(&self) -> Option<PullRequest<'a>> {
        if let Some(index) = self.hovered_index {
            let item: &PullRequest<'a> = &self.items[index];
            return Some(item.clone());
        }
        None
    }
}

impl<'a> Component for PullRequestList<'a> {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Move(direction) => match direction {
                Direction::Up => self.hover_up(),
                Direction::Down => self.hover_down(),
                _ => {}
            },
            _ => {}
        }
    }

    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error> {
        let widths = vec![
            self.items
                .iter()
                .fold(0, |a, x| usize::max(a, x.number.to_string().len())),
            self.items
                .iter()
                .fold(0, |a, x| usize::max(a, x.title.len())),
            self.items
                .iter()
                .fold(0, |a, x| usize::max(a, x.owner.login.len())),
            self.items
                .iter()
                .fold(0, |a, x| usize::max(a, x.state.len())),
        ];

        for (i, item) in &mut self.items.iter().enumerate() {
            write!(screen, "{}", ui::goto(2, i + 1))?;

            if let Some(index) = self.hovered_index {
                if index == i {
                    write!(screen, "{}", Bg(palette.bg_highlight))?;
                }
            }

            write!(
                screen,
                "{fg}#{num:<width$} ",
                fg = Fg(palette.fg_alt2),
                num = item.number,
                width = widths[0]
            )?;

            write!(
                screen,
                "{fg}{title:<width$} ",
                fg = Fg(palette.fg_alt3),
                title = item.title,
                width = widths[1],
            )?;

            write!(
                screen,
                "{fg}{login:<width$} ",
                fg = Fg(palette.fg_alt4),
                login = item.owner.login,
                width = widths[2],
            )?;

            write!(
                screen,
                "{fg}{state:<width$}",
                fg = Fg(palette.fg_alt5),
                state = item.state,
                width = widths[3],
            )?;

            write!(screen, "{}{}", ui::clear_rest(), palette.dual_reset())?;
        }

        self.dirty = false;
        Ok(())
    }
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
