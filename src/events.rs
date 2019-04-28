#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug)]
pub enum Event {
    Quit,
    Move(Direction),
    Back,
}
