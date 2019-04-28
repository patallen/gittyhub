use super::Palette;
use crate::events::Event;
use std::io::{self, Write};

pub trait Component {
    fn render(&mut self, screen: &mut Write, palette: &Palette) -> Result<(), io::Error>;
    fn handle_event(&mut self, event: Event);
    fn dirty(&self) -> bool;
}
