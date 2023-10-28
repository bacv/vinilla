mod cell;
mod grid;

pub use cell::Cell;
pub use grid::{Column, Grid, GridIterator, Indexed, Line};

pub struct Term {
    grid: Grid<Cell>,
}

impl Term {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self {
            grid: Grid::new(columns, lines),
        }
    }

    pub fn input(&mut self, c: char) {
        let fg = self.grid.cursor.template.fg;
        let bg = self.grid.cursor.template.bg;
        let cursor_cell = self.grid.cursor_cell();

        cursor_cell.c = c;
        cursor_cell.fg = fg;
        cursor_cell.bg = bg;
    }

    pub fn goto(&mut self, line: i32, col: usize) {
        self.grid.cursor.point.line = Line(line);
        self.grid.cursor.point.column = Column(col);
    }

    pub fn renderable_content(&self) -> GridIterator<Cell> {
        self.grid.display_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn display_line() {
        let mut term = Term::new(2, 2);
        term.goto(0, 0);
        term.input('h');
        term.goto(0, 1);
        term.input('i');
        term.goto(1, 1);
        term.input('!');

        let mut display = term.renderable_content();
        assert_eq!(display.next().unwrap().c, 'h');
        assert_eq!(display.next().unwrap().c, 'i');
        assert_eq!(display.next().unwrap().c, ' ');
        assert_eq!(display.next().unwrap().c, '!');
    }
}
