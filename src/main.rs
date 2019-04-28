#[macro_use]
extern crate log;
extern crate chrono;

mod app;
mod events;
mod mock;
mod models;
mod ui;
mod views;

use app::{App, AppState};
use events::EventPump;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::screen::*;
use ui::Palette;

fn main() {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let palette = Palette::default();

    let user = mock::user("patallen").unwrap();
    let state = AppState::for_user(user.clone());

    let mut app = App::new(palette, state);

    app.render(&mut screen).unwrap();

    let event_loop = EventPump::new();

    'main: loop {
        if let Some(event) = event_loop.next() {
            app.handle_event(event);
        }

        if app.should_quit() {
            break 'main;
        }

        app.render(&mut screen).unwrap();
    }

    write!(screen, "{}", ui::cursor_show()).unwrap();
    screen.flush().unwrap();
}
