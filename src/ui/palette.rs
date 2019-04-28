use termion::color::{Bg, Color, Fg, Green, Magenta, Reset, Rgb};

pub struct Palette<'a> {
    pub bg_main: &'a Color,
    pub bg_highlight: &'a Color,
    pub bg_selected: &'a Color,
    pub bg_alt1: &'a Color,
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
            bg_selected: &Rgb(60, 50, 20),
            fg_normal: &Rgb(250, 250, 250),
            bg_alt1: &Rgb(36, 36, 36),
            fg_alt1: &Rgb(30, 30, 240),
            fg_alt2: &Rgb(140, 130, 255),
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
