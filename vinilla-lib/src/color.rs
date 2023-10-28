use vte::ansi::{Color, NamedColor};

pub(crate) trait ColorExt {
    fn to_u16(&self) -> u16;
}

impl ColorExt for Color {
    fn to_u16(&self) -> u16 {
        match self {
            Color::Named(named) => *named as u16,
            Color::Spec(_) => 0, // TODO
            Color::Indexed(index) => {
                const NAMED_COLORS_COUNT: u16 = NamedColor::Black as u16 + 1;
                NAMED_COLORS_COUNT + *index as u16
            }
        }
    }
}
