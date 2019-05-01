#[macro_use]
extern crate log;

use std::io;
use std::io::Write;
use termion::color;
use termion::cursor;

pub fn goto(screen: &mut Write, x: usize, y: usize) -> Result<(), io::Error> {
    write!(screen, "{}", cursor::Goto(x as u16, y as u16))?;
    Ok(())
}

pub fn draw_segment(
    screen: &mut Write,
    x: usize,
    y: usize,
    width: usize,
    color: &color::Color,
) -> Result<(), io::Error> {
    debug!(
        "Draw {:?} from ({x1}, {y}) to ({x2}, {y})",
        color,
        x1 = x,
        y = y,
        x2 = width + x
    );
    goto(screen, x, y)?;
    let text = format!("{},{}@{}", x, y, width);
    let output = format!("{:<width$}", text = text, width = width);
    write!(screen, "{}", color::Bg(color))?;
    write!(screen, "{}", output)
}

pub fn fill_context(
    screen: &mut Write,
    context: &Context,
    color: &color::Color,
) -> Result<(), io::Error> {
    for line in context.top()..context.bottom() + 1 {
        draw_segment(screen, context.left(), line, context.width, color)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Magenta,
    Black,
    White,
    Cyan,
}

#[derive(Debug)]
pub struct Pane {
    layout: Layout,
    color: Color,
}

impl Pane {
    pub fn new(layout: Layout, color: Color) -> Self {
        Pane { layout, color }
    }
}

#[derive(Debug)]
pub enum Format {
    Rows,
    Columns,
}

#[derive(Debug)]
pub struct Layout {
    pub format: Format,
    pub panes: Vec<Pane>,
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            format: Format::Rows,
            panes: Vec::new(),
        }
    }
}

impl Layout {
    pub fn add_pane(mut self, pane: Pane) -> Self {
        self.panes.push(pane);
        self
    }
}

#[derive(Debug)]
pub struct Context {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Context {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Context {
            x,
            y,
            width,
            height,
        }
    }

    pub fn top(&self) -> usize {
        self.y
    }

    pub fn bottom(&self) -> usize {
        self.y + self.height - 1
    }

    pub fn left(&self) -> usize {
        self.x
    }

    pub fn right(&self) -> usize {
        self.x + self.width - 1
    }
}

pub fn contexts_offset(
    context: &Context,
    offset: usize,
    pane_count: usize,
    format: &Format,
) -> Context {
    match format {
        Format::Rows => {
            let height = context.height / pane_count;
            let bottom = context.top() + height * offset;
            Context::new(context.left(), bottom, context.width, height)
        }
        Format::Columns => {
            let width = context.width / pane_count;
            let left = context.left() + width * offset;
            Context::new(left, context.top(), width, context.height)
        }
    }
}

pub fn generate_layout(pane: &Pane, context: Context) -> Vec<(Context, &Color)> {
    let mut segments = vec![];

    let pane_count = pane.layout.panes.len();

    for (i, p) in pane.layout.panes.iter().enumerate() {
        segments.append(&mut generate_layout(
            &p,
            contexts_offset(&context, i, pane_count, &pane.layout.format),
        ));
    }

    if segments.len() > 0 {
        return segments;
    }
    vec![(context, &pane.color)]
}
