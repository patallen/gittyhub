#[macro_use]
extern crate log;

mod mock;
mod models;
mod ui;

use ui::Palette;

use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let user = mock::user("patallen");

    let pulls = mock::pull_requests(user.unwrap());
    let palette = Palette::default();

    write!(screen, "{}{}", palette.dual_reset(), ui::clear_all()).unwrap();

    for (i, pull) in pulls.unwrap().iter().enumerate() {
        write!(
            screen,
            "{}{}{}{}\n",
            ui::goto(1, i + 1),
            ui::color_fg(&format!("#{} ", pull.number), palette.fg_alt4),
            ui::color_fg(pull.title, palette.fg_alt2),
            ui::color_fg(&format!(" ({})", pull.owner.login), palette.fg_alt3),
        )
        .unwrap();
    }
    write!(screen, "{}", ui::cursor_hide()).unwrap();

    screen.flush().unwrap();

    'main: loop {
        let stdin = stdin();
        for event in stdin.events() {
            let event = event.unwrap();
            match event {
                Event::Key(Key::Esc) => break 'main,
                Event::Key(Key::Backspace) => debug!("[input] PRESSED BACKSPACE"),
                Event::Key(Key::Char(ch)) => match ch {
                    'q' => break 'main,
                    'j' => debug!("[input] PRESSED DOWN"),
                    'k' => debug!("[input] PRESSED UP"),
                    'h' => debug!("[input] PRESSED LEFT"),
                    'l' => debug!("[input] PRESSED RIGHT"),
                    _ => {}
                },
                _ => {}
            };
        }
    }
    write!(screen, "{}", ui::cursor_show()).unwrap();
    screen.flush().unwrap();
}
