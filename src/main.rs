#[macro_use]
extern crate log;

mod app;
mod mock;
mod models;
mod ui;

use app::{App, AppState};
use ui::Palette;

use app::PullRequestList;
use app::Render;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn main() {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let palette = Palette::default();

    let user = mock::user("patallen").unwrap();
    let state = AppState::for_user(user.clone());
    let pulls = mock::pull_requests(user.clone()).unwrap();
    let views: Vec<Box<dyn Render>> = vec![Box::new(PullRequestList::new(pulls))];

    let mut app = App::new(palette, state, views);

    app.render(&mut screen).unwrap();

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
