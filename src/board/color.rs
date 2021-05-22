#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        if *self == Color::White { Color::Black } else { Color::White }
    }
}
