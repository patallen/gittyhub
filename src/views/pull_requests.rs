use crate::events::{Direction, Event};
use crate::models::PullRequest;
use crate::ui::{self, Component, Palette};
use std::io::{self, Write};
use termion::color::{Bg, Fg};

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
