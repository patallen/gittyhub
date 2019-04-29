#[macro_use]
extern crate log;
extern crate chrono;

mod app;
mod events;
mod gh;
mod ui;
mod views;

use app::{App, AppState};
use events::EventPump;
use std::fs;
use std::io;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::screen::*;
use ui::Palette;

fn api_key_from_file<'a>(filename: &'a str) -> String {
    fs::read_to_string(filename).expect(&format!(
        r##"
There was an error while attempting to read your Github API key.

Ensure that you have a single-line file named '{}' in root
directory containing a key obtained from Github.
"##,
        filename
    ))
}

fn main() -> Result<(), io::Error> {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let api_key = api_key_from_file(".apikey");

    let mut screen = AlternateScreen::from(stdout().into_raw_mode()?);

    let palette = Palette::default();

    let user = gh::User {
        url: "https://github.com/patallen".into(),
        site_admin: false,
        avatar_url: "".into(),
        id: 0,
        login: "patallen".into(),
    };
    let state = AppState::for_user(user.clone());

    let client = gh::Client::new(&api_key);

    let mut app = App::new(palette, state, client);

    app.render(&mut screen)?;

    let event_loop = EventPump::new();

    'main: loop {
        if let Some(event) = event_loop.next() {
            app.handle_event(event);
        }

        if app.should_quit() {
            break 'main;
        }

        app.render(&mut screen)?;
    }

    write!(screen, "{}", ui::cursor_show())?;
    screen.flush()?;;
    Ok(())
}
