mod row;
mod storage;

use core::{
    cmp::Ordering,
    ops::{Deref, Index, IndexMut},
};

use self::{row::Row, storage::Storage};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Column(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Line(pub i32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Point<L = Line, C = Column> {
    pub line: L,
    pub column: C,
}

impl<L: Ord, C: Ord> PartialOrd for Point<L, C> {
    fn partial_cmp(&self, other: &Point<L, C>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: Ord, C: Ord> Ord for Point<L, C> {
    fn cmp(&self, other: &Point<L, C>) -> Ordering {
        match (self.line.cmp(&other.line), self.column.cmp(&other.column)) {
            (Ordering::Equal, ord) | (ord, _) => ord,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cursor<T> {
    /// The location of this cursor.
    pub point: Point,

    /// Template cell when using this cursor.
    pub template: T,
}

pub struct Grid<T> {
    /// Current cursor for writing data.
    pub cursor: Cursor<T>,

    /// Last saved cursor.
    pub saved_cursor: Cursor<T>,

    /// Lines in the grid. Each row holds a list of cells corresponding to the
    /// columns in that row.
    raw: Storage<T>,

    /// Number of columns.
    columns: usize,

    /// Number of visible lines.
    lines: usize,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self {
            cursor: Default::default(),
            saved_cursor: Default::default(),
            raw: Storage::with_capacity(lines, columns),
            columns,
            lines,
        }
    }

    pub fn cursor_cell(&mut self) -> &mut T {
        let point = self.cursor.point;
        &mut self.raw[point.line][point.column]
    }

    pub fn display_iter(&self) -> GridIterator<'_, T> {
        let start = Point {
            line: Line(-1),
            column: self.last_column(),
        };
        let end = Point {
            line: Line((self.lines - 1) as i32),
            column: self.last_column(),
        };

        GridIterator {
            grid: self,
            point: start,
            end,
        }
    }
}

impl<T> Grid<T> {
    pub fn last_column(&self) -> Column {
        Column(self.columns - 1)
    }
}

impl<T> Index<Line> for Grid<T> {
    type Output = Row<T>;

    #[inline]
    fn index(&self, index: Line) -> &Row<T> {
        &self.raw[index]
    }
}

impl<T> IndexMut<Line> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Line) -> &mut Row<T> {
        &mut self.raw[index]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &T {
        &self[point.line][point.column]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut T {
        &mut self[point.line][point.column]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Indexed<T> {
    pub point: Point,
    pub cell: T,
}

impl<T> Deref for Indexed<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.cell
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    point: Point,
    end: Point,
}

impl<'a, T> GridIterator<'a, T> {
    /// Current iteratior position.
    pub fn point(&self) -> Point {
        self.point
    }

    /// Cell at the current iteratior position.
    pub fn cell(&self) -> &'a T {
        &self.grid[self.point]
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = Indexed<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop once we've reached the end of the grid.
        if self.point >= self.end {
            return None;
        }

        match self.point {
            Point { column, .. } if column == self.grid.last_column() => {
                self.point.column = Column(0);
                self.point.line.0 += 1;
            }
            _ => self.point.column.0 += 1,
        }

        Some(Indexed {
            cell: &self.grid[self.point],
            point: self.point,
        })
    }
}
