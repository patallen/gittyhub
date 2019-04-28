use termion::clear;
use termion::color::{Bg, Color, Fg};
use termion::cursor;

pub fn color_bg(string: &str, color: &Color) -> String {
    format!("{}{}", Bg(color), string)
}

pub fn color_fg(string: &str, color: &Color) -> String {
    format!("{}{}", Fg(color), string)
}

pub fn goto(x: usize, y: usize) -> String {
    format!("{}", cursor::Goto(x as u16, y as u16))
}

pub fn cursor_hide() -> String {
    format!("{}", cursor::Hide)
}

pub fn cursor_show() -> String {
    format!("{}", cursor::Show)
}

pub fn clear_rest() -> String {
    format!("{}", clear::UntilNewline)
}

pub fn clear_all() -> String {
    format!("{}", clear::All)
}
