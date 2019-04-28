use super::models::{PullRequest, User};
use super::ui::Palette;
use std::io;
use std::io::Write;
use termion::color::Fg;

use super::ui;
pub struct AppState {
    dirty: bool,
    pub user: User,
}

impl AppState {
    pub fn toggle_dirty(&mut self) {
        self.dirty = !self.dirty;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn for_user(user: User) -> AppState {
        AppState { dirty: true, user }
    }
}

pub struct App<'a> {
    palette: Palette<'a>,
    state: AppState,
    views: Vec<Box<dyn Render>>,
}

pub trait Render {
    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error>;
}

pub struct PullRequestList<'a> {
    items: Vec<PullRequest<'a>>,
}

impl<'a> PullRequestList<'a> {
    pub fn new(pull_requests: Vec<PullRequest<'a>>) -> PullRequestList<'a> {
        PullRequestList {
            items: pull_requests,
        }
    }
}

impl<'a> Render for PullRequestList<'a> {
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
        ];

        for (i, item) in &mut self.items.iter().enumerate() {
            write!(screen, "{}", ui::goto(2, i + 1))?;

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
                "{fg}{login:<width$}",
                fg = Fg(palette.fg_alt4),
                login = item.owner.login,
                width = widths[2],
            )?;
        }

        Ok(())
    }
}

impl<'a> App<'a> {
    pub fn new(palette: Palette<'a>, state: AppState, views: Vec<Box<dyn Render>>) -> App<'a> {
        App {
            palette,
            state,
            views,
        }
    }

    pub fn render(&mut self, screen: &mut Write) -> Result<(), io::Error> {
        write!(screen, "{}{}", self.palette.dual_reset(), ui::clear_all()).unwrap();
        self.views[0].render(screen, &self.palette)?;
        write!(screen, "{}", ui::cursor_hide())?;
        screen.flush()
    }
}
