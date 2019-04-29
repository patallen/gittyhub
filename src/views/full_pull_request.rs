use crate::events::{Command, Event};
use crate::gh::PullRequest;
use crate::ui::Component;
use crate::ui::{self, Palette};
use std::io::{self, Write};
use termion::color::Fg;

pub struct FullPullRequest {
    dirty: bool,
    item: Box<PullRequest>,
}

impl FullPullRequest {
    pub fn new(item: Box<PullRequest>) -> FullPullRequest {
        FullPullRequest { dirty: true, item }
    }
}

impl<'a> Component<'a> for FullPullRequest {
    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error> {
        write!(screen, "{}", ui::goto(1, 1))?;
        write!(
            screen,
            "{}#{} {}{}\n\r",
            Fg(palette.fg_alt5),
            self.item.number,
            Fg(palette.fg_alt3),
            self.item.title
        )?;
        write!(
            screen,
            "{}{}\n\r",
            Fg(palette.fg_alt1),
            self.item.user.login
        )?;
        write!(screen, "{}{}", Fg(palette.fg_normal), self.item.body)?;
        Ok(())
    }

    fn handle_event(&mut self, _event: Event) {}

    fn dirty(&self) -> bool {
        return self.dirty;
    }

    fn select(&self) -> Option<Command> {
        None
    }
}
