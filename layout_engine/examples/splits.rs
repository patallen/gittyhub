#[macro_use]
extern crate log;

use layout_engine::Color;
use layout_engine::*;
use std::io::{stdin, stdout, Read, Write};
use termion::color;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

fn match_color(color: &layout_engine::Color) -> Result<&color::Color, String> {
    match *color {
        Color::Blue => Ok(&color::Blue),
        Color::Green => Ok(&color::Green),
        Color::Magenta => Ok(&color::Magenta),
        Color::Cyan => Ok(&color::Cyan),
        Color::Black => Ok(&color::Black),
        Color::White => Ok(&color::White),
        _ => Err("invalid color".into()),
    }
}

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    // let mut screen = stdout();

    let (width, height) = termion::terminal_size().unwrap();
    debug!("Size: {}, {}", width, height);

    let main_pane = Pane::new(
        Layout::default()
            .add_pane(Pane::new(
                Layout {
                    panes: vec![
                        Pane::new(
                            Layout {
                                panes: vec![
                                    Pane::new(Layout::default(), Color::Magenta),
                                    Pane::new(Layout::default(), Color::Cyan),
                                ],
                                format: Format::Rows,
                            },
                            Color::Blue,
                        ),
                        Pane::new(Layout::default(), Color::Green),
                    ],
                    format: Format::Columns,
                },
                Color::Blue,
            ))
            .add_pane(Pane::new(Layout::default(), Color::White)),
        Color::Blue,
    );
    let context = Context::new(1, 1, width as usize, height as usize);
    let segments = generate_layout(&main_pane, context);

    for (context, color) in segments {
        let color = match_color(color).unwrap();
        fill_context(&mut screen, &context, color).unwrap();
        screen.flush().unwrap();
    }

    'main: loop {
        for byte in stdin().bytes() {
            match byte {
                Ok(b'q') => break 'main,
                _ => {}
            };
        }
    }
}
