use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

/// Represents direction of keyboard movements
#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

/// Represents an event from the EventPump
#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    Move(Direction),
    Back,
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

/// Simple EventPump for receiving events asynchronously.
pub struct EventLoop {
    rx: Receiver<Event>,
}

impl EventLoop {
    /// Fetch the next event from the receiver.
    /// Example:
    /// ```
    /// let mut pump = EventLoop::new();
    ///
    /// loop {
    ///     if let Some(event) = pump.next() {
    ///         // do cool event handling stuff here
    ///     }
    /// }
    /// ```
    pub fn next(&self) -> Option<Event> {
        match self.rx.recv() {
            Ok(event) => Some(event),
            _ => None,
        }
    }

    /// Create a new EventPump
    /// Example:
    /// ```
    /// let mut pump = EventLoop::new();
    /// ```
    pub fn new() -> EventLoop {
        let (tx, rx) = mpsc::channel();
        let event_loop = EventLoop { rx: rx };

        thread::spawn(move || loop {
            for event in stdin().events() {
                use termion::event::Event as Te;

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
                    tx.send(event.clone()).unwrap();
                }
            }
        });

        event_loop
    }
}
