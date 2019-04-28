use crate::events::{Command, Direction, Event};
use crate::models::PullRequest;
use crate::ui::{self, Component, Palette};
use std::io::{self, Write};
use termion::color::{Bg, Fg};

pub struct PullRequestList {
    dirty: bool,
    items: Vec<PullRequest>,
    hovered_index: Option<usize>,
}

impl PullRequestList {
    pub fn new(pull_requests: Vec<PullRequest>) -> PullRequestList {
        PullRequestList {
            dirty: true,
            items: pull_requests,
            hovered_index: None,
        }
    }

    pub fn into_items(self) -> Vec<PullRequest> {
        self.items
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

    pub fn draw_header(
        &self,
        screen: &mut Write,
        widths: &Vec<usize>,
        palette: &Palette,
    ) -> Result<(), io::Error> {
        write!(screen, "{}{}", ui::goto(1, 1), Bg(palette.bg_alt1))?;

        let mut column_widths = widths.clone();
        column_widths[0] += 1;

        let column_names = vec!["Num", "Title", "Owner", "State"];
        for (width, name) in column_widths.iter().zip(column_names) {
            write!(screen, "{name:<width$} ", name = name, width = width)?;
        }

        write!(screen, "{}{}", ui::clear_rest(), palette.dual_reset())?;
        write!(screen, "\n\r")
    }

    pub fn draw_line(
        &self,
        screen: &mut Write,
        pr: &PullRequest,
        widths: &Vec<usize>,
        palette: &Palette,
        hovered: bool,
    ) -> Result<(), io::Error> {
        if hovered {
            write!(screen, "{}", Bg(palette.bg_highlight))?;
        }
        write!(
            screen,
            "{fg}#{num:<width$} ",
            fg = Fg(palette.fg_alt2),
            num = pr.number,
            width = widths[0]
        )?;

        write!(
            screen,
            "{fg}{title:<width$} ",
            fg = Fg(palette.fg_alt3),
            title = pr.title,
            width = widths[1],
        )?;

        write!(
            screen,
            "{fg}{login:<width$} ",
            fg = Fg(palette.fg_alt4),
            login = pr.owner.login,
            width = widths[2],
        )?;

        write!(
            screen,
            "{fg}{state:<width$}",
            fg = Fg(palette.fg_alt5),
            state = pr.state,
            width = widths[3],
        )?;
        write!(screen, "{}", ui::clear_rest())?;
        write!(screen, "{}\n\r", palette.dual_reset())
    }
}

impl<'a> Component<'a> for PullRequestList {
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

    fn select(&self) -> Option<Command> {
        if let Some(index) = self.hovered_index {
            let item: PullRequest = self.items[index].clone();
            return Some(Command::ShowPull(Box::new(item)));
        }
        None
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

        self.draw_header(screen, &widths, palette)?;
        for (i, item) in &mut self.items.iter().enumerate() {
            let hovered = self.hovered_index.is_some() && self.hovered_index.unwrap() == i;
            self.draw_line(screen, item, &widths, palette, hovered)?;
        }

        self.dirty = false;
        Ok(())
    }
}
