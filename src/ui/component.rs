use super::Palette;
use crate::events::{Command, Event};
use std::io::{self, Write};

pub trait Component<'a> {
    fn select(&self) -> Option<Command>;
    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error>;
    fn handle_event(&mut self, event: Event);
    fn dirty(&self) -> bool;
}
