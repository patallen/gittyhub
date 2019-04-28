#[macro_use]
extern crate log;

mod app;
mod events;
mod mock;
mod models;
mod ui;

use app::{App, AppState};
use ui::Palette;

use app::Component;
use app::PullRequestList;
use events::{Direction, Event};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
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
    let views: Vec<Box<dyn Component>> = vec![Box::new(PullRequestList::new(pulls))];

    let mut app = App::new(palette, state, views);

    app.render(&mut screen).unwrap();

    'main: loop {
        let stdin = stdin();

        use termion::event::Event as Te;

        for event in stdin.events() {
            let event = event.unwrap();
            let app_event = match event {
                Te::Key(Key::Esc) => Some(Event::Quit),
                Te::Key(Key::Backspace) => Some(Event::Back),
                Te::Key(Key::Char(ch)) => match ch {
                    'q' => Some(Event::Quit),
                    'j' => Some(Event::Move(Direction::Down)),
                    'k' => Some(Event::Move(Direction::Up)),
                    'h' => Some(Event::Move(Direction::Left)),
                    'l' => Some(Event::Move(Direction::Right)),
                    _ => None,
                },
                _ => None,
            };

            if let Some(event) = app_event {
                app.handle_event(event);
            }

            if app.should_quit() {
                break 'main;
            }

            app.render(&mut screen).unwrap();
        }
    }

    write!(screen, "{}", ui::cursor_show()).unwrap();
    screen.flush().unwrap();
}
