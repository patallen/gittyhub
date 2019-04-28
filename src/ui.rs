use termion::clear;
use termion::color::{Bg, Color, Fg, Green, Magenta, Reset, Rgb};
use termion::cursor;
use termion::cursor::Goto;

pub struct Palette<'a> {
    pub bg_main: &'a Color,
    pub bg_highlight: &'a Color,
    pub bg_selected: &'a Color,
    pub fg_normal: &'a Color,
    pub fg_alt1: &'a Color,
    pub fg_alt2: &'a Color,
    pub fg_alt3: &'a Color,
    pub fg_alt4: &'a Color,
    pub fg_alt5: &'a Color,
}

impl<'a> Default for Palette<'a> {
    fn default() -> Palette<'a> {
        Palette {
            bg_main: &Rgb(23, 23, 23),
            bg_highlight: &Rgb(40, 40, 40),
            bg_selected: &Rgb(100, 100, 100),
            fg_normal: &Rgb(250, 250, 250),
            fg_alt1: &Rgb(30, 30, 240),
            fg_alt2: &Rgb(60, 80, 250),
            fg_alt3: &Magenta,
            fg_alt4: &Rgb(255, 20, 147),
            fg_alt5: &Green,
        }
    }
}

impl<'a> Palette<'a> {
    pub fn dual_reset(&self) -> String {
        let fg = self.fg_reset();
        let bg = self.bg_reset();
        format!("{}{}", fg, bg)
    }

    pub fn fg_reset(&self) -> String {
        format!("{}{}", Fg(Reset), Fg(self.fg_normal))
    }

    pub fn bg_reset(&self) -> String {
        format!("{}{}", Bg(Reset), Bg(self.bg_main))
    }
}

pub fn color_bg(string: &str, color: &Color) -> String {
    format!("{}{}", Bg(color), string)
}

pub fn color_fg(string: &str, color: &Color) -> String {
    format!("{}{}", Fg(color), string)
}

pub fn goto(x: usize, y: usize) -> String {
    format!("{}", Goto(x as u16, y as u16))
}

pub fn clear_all() -> String {
    format!("{}", clear::All)
}

pub fn cursor_hide() -> String {
    format!("{}", cursor::Hide)
}

pub fn cursor_show() -> String {
    format!("{}", cursor::Show)
}
