#[macro_use]
extern crate log;
extern crate chrono;

use std::io;
use std::io::{stdin, stdout, Write};
use std::process;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError};
use std::thread;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

#[derive(Debug, Clone)]
pub enum Clear {
    Left,
    Right,
    Line,
    All,
}

#[derive(Debug, Clone)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Gray,
    Green,
    Magenta,
    Red,
    White,
}

#[derive(Debug, Clone)]
pub enum Command {
    Write(String),
    Goto(u16, u16),
    SetContext(u16, u16, u16, u16),
    Clear(Clear),
    SetFg(Color),
    SetBg(Color),
}

#[derive(Debug, Clone)]
pub enum RenderEvent {
    Debug(String),
    Render,
    Draw(Command),
}

unsafe impl Send for RenderEvent {}
unsafe impl Sync for RenderEvent {}

pub struct Listener<E> {
    r: Receiver<E>,
}

impl<E: Send> Listener<E> {
    pub fn next(&self) -> Result<E, RecvError> {
        self.r.recv()
    }
    pub fn new(r: Receiver<E>) -> Self
    where
        E: Send + Sync,
    {
        Listener { r }
    }
}

struct Renderer<W> {
    out: W,
    events: Listener<RenderEvent>,
    queue: Vec<RenderEvent>,
}

use std::fmt;
fn convert_clear<'a>(clear: Clear) -> Result<&'a fmt::Display, ()> {
    let result: &fmt::Display = match clear {
        Clear::All => &termion::clear::All,
        Clear::Right => &termion::clear::UntilNewline,
        _ => panic!("Clear {:?} not implemented"),
    };

    Ok(result)
}

fn convert_color<'a>(color: Color) -> Result<&'a termion::color::Color, ()> {
    let result: &termion::color::Color = match color {
        Color::Green => &termion::color::Green,
        Color::Magenta => &termion::color::Magenta,
        Color::Red => &termion::color::Red,
        Color::Blue => &termion::color::Blue,
        Color::Cyan => &termion::color::Cyan,
        Color::White => &termion::color::White,
        Color::Black => &termion::color::Black,
        _ => panic!("Color {:?} not implemented"),
    };
    Ok(result)
}

fn set_bg(screen: &mut Write, color: Color) -> Result<(), io::Error> {
    let color = convert_color(color).unwrap();
    write!(screen, "{}", termion::color::Bg(color))
}

fn set_fg(screen: &mut Write, color: Color) -> Result<(), io::Error> {
    let color = convert_color(color).unwrap();
    write!(screen, "{}", termion::color::Fg(color))
}

pub fn goto(screen: &mut Write, x: u16, y: u16) -> Result<(), io::Error> {
    write!(screen, "{}", termion::cursor::Goto(x, y))
}

fn debug(screen: &mut Write, message: String) -> Result<(), io::Error> {
    let (_, height) = termion::terminal_size().unwrap();
    goto(screen, 1, height)?;
    set_bg(screen, Color::White)?;
    set_fg(screen, Color::Black)?;
    write!(screen, "{}", message)?;
    write!(screen, "{}", termion::clear::UntilNewline)?;
    screen.flush()
}

fn clear(screen: &mut Write, clear_type: Clear) -> Result<(), io::Error> {
    write!(screen, "{}", convert_clear(clear_type).unwrap())
}

fn text(screen: &mut Write, text: String) -> Result<(), io::Error> {
    write!(screen, "{}", text)
}

fn handle_event(screen: &mut Write, event: RenderEvent) -> Result<(), io::Error> {
    match event {
        RenderEvent::Draw(command) => match command {
            Command::Clear(clear_type) => clear(screen, clear_type)?,
            Command::Goto(x, y) => goto(screen, x, y)?,
            Command::Write(string) => text(screen, string)?,
            Command::SetBg(color) => set_bg(screen, color)?,
            _ => panic!("Not implemented"),
        },
        _ => panic!("This event should have been handled by the renderer."),
    };
    Ok(())
}

impl<W: Write> Renderer<W> {
    pub fn new(out: W, listener: Listener<RenderEvent>) -> Self {
        Renderer {
            out,
            events: listener,
            queue: Vec::new(),
        }
    }

    fn render(&mut self) -> Result<(), io::Error> {
        debug!("[Renderer]: rendering");
        for event in self.queue.drain(..) {
            handle_event(&mut self.out, event)?;
        }
        self.out.flush()
    }

    fn receive_event(&mut self, event: RenderEvent) -> Result<(), io::Error> {
        match event {
            RenderEvent::Debug(message) => debug(&mut self.out, message)?,
            RenderEvent::Render => self.render()?,
            _ => self.queue.push(event.clone()),
        };
        Ok(())
    }

    pub fn listen(&mut self) {
        loop {
            if let Ok(event) = self.events.next() {
                self.receive_event(event).unwrap();
            }
        }
    }
}

pub fn main() -> Result<(), io::Error> {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let (send_to_render, render_receiver) = mpsc::channel::<RenderEvent>();

    thread::spawn(move || {
        let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let listener = Listener::new(render_receiver);
        let mut renderer = Renderer::new(screen, listener);
        renderer.listen();
    });

    let mut buff = String::new();

    use termion::event::{Event, Key};
    'main: loop {
        let events = stdin().events();
        for event in events {
            match event {
                Ok(evt) => {
                    match evt {
                        Event::Key(key) => match key {
                            Key::Backspace => {
                                buff.pop();
                            }
                            Key::Esc => {
                                process::exit(0);
                            }
                            Key::Char('\n') => {
                                let b = buff.clone();
                                let tokens = b.split(' ').collect::<Vec<&str>>();
                                buff.clear();
                                match tokens[0] {
                                    "text" => {
                                        let string = tokens[1..]
                                            .iter()
                                            .map(|t| *t)
                                            .collect::<Vec<&str>>()
                                            .join(" ");

                                        send_to_render
                                            .send(RenderEvent::Draw(Command::Write(string)))
                                            .unwrap()
                                    }
                                    "goto" => {
                                        if !tokens.len() == 3 {
                                            warn!("Not enough arguments")
                                        }
                                        let x = str::parse::<u16>(tokens[1]).unwrap();
                                        let y = str::parse::<u16>(tokens[2]).unwrap();
                                        send_to_render
                                            .send(RenderEvent::Draw(Command::Goto(x, y)))
                                            .unwrap();
                                    }
                                    "clear" => send_to_render
                                        .send(RenderEvent::Draw(Command::Clear(Clear::All)))
                                        .unwrap(),
                                    "render" => send_to_render.send(RenderEvent::Render).unwrap(),
                                    "set" => match tokens[1] {
                                        "fg" => match tokens[2] {
                                            "cyan" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetFg(
                                                    Color::Cyan,
                                                )))
                                                .unwrap(),
                                            "blue" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetFg(
                                                    Color::Blue,
                                                )))
                                                .unwrap(),
                                            "magenta" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetFg(
                                                    Color::White,
                                                )))
                                                .unwrap(),
                                            "green" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetFg(
                                                    Color::Green,
                                                )))
                                                .unwrap(),
                                            color => warn!("{} is not a valid color.", color),
                                        },
                                        "bg" => match tokens[2] {
                                            "cyan" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetBg(
                                                    Color::Cyan,
                                                )))
                                                .unwrap(),
                                            "blue" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetBg(
                                                    Color::Blue,
                                                )))
                                                .unwrap(),
                                            "magenta" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetBg(
                                                    Color::White,
                                                )))
                                                .unwrap(),
                                            "green" => send_to_render
                                                .send(RenderEvent::Draw(Command::SetBg(
                                                    Color::Green,
                                                )))
                                                .unwrap(),
                                            color => warn!("{} is not a valid color.", color),
                                        },
                                        target => warn!("{} is not a valid target.", target),
                                    },
                                    command => warn!("{} is not a valid command.", command),
                                };
                                send_to_render
                                    .send(RenderEvent::Debug(buff.clone()))
                                    .unwrap();
                            } // <- Key::Char('\n')
                            Key::Char(c) => {
                                &buff.push(c);
                                send_to_render
                                    .send(RenderEvent::Debug(buff.to_owned()))
                                    .unwrap();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            };

            debug!("{}", buff);
        }
    }
}
