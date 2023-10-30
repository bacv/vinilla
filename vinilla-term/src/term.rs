use crate::{Cell, Column, Grid, GridIterator, Line};

pub struct Term {
    grid: Grid<Cell>,
}

impl Term {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self {
            grid: Grid::new(lines, columns),
        }
    }

    pub fn input(&mut self, c: char) {
        let fg = self.grid.cursor.template.fg;
        let bg = self.grid.cursor.template.bg;
        let cursor_cell = self.grid.cursor_cell();

        cursor_cell.c = c;
        cursor_cell.fg = fg;
        cursor_cell.bg = bg;

        self.grid.cursor.point.column.0 += 1;
    }

    pub fn goto(&mut self, line: i32, col: usize) {
        self.grid.cursor.point.line = Line(line);
        self.grid.cursor.point.column = Column(col);
    }

    pub fn renderable_content(&self) -> GridIterator<Cell> {
        self.grid.display_iter()
    }

    pub fn new_line(&mut self) {
        if self.grid.cursor.point.line.0 < self.grid.lines as i32 - 1 {
            self.grid.cursor.point.line.0 += 1;
        }
        self.grid.cursor.point.column = Column(0);
    }

    pub fn carriage_return(&mut self) {
        self.grid.cursor.point.column = Column(0);
    }

    pub fn backspace(&mut self) {
        if self.grid.cursor.point.column.0 > 0 {
            self.grid.cursor.point.column.0 -= 1;
            self.grid.cursor_cell().reset();
        }
    }

    pub fn move_cursor_right(&mut self, n: usize) {
        self.grid.cursor.point.column.0 =
            (self.grid.cursor.point.column.0 + n).min(self.grid.columns - 1);
    }

    pub fn move_cursor_up(&mut self, n: usize) {
        self.grid.cursor.point.line.0 = self.grid.cursor.point.line.0.saturating_sub(n as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn input_one_by_one() {
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

    #[test]
    fn input_line_by_line() {
        let mut term = Term::new(2, 2);
        term.goto(0, 0);
        term.input('h');
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
