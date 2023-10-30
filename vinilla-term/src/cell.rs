use vte::ansi::{Color, NamedColor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn reset(&mut self) {
        *self = Cell::default();
    }
}

impl Default for Cell {
    #[inline]
    fn default() -> Cell {
        Cell {
            c: ' ',
            bg: Color::Named(NamedColor::Background),
            fg: Color::Named(NamedColor::Foreground),
        }
    }
}
