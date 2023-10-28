mod color;

use color::ColorExt;
use vinilla_term::{Cell as TermCell, Indexed, Term};

#[repr(C)]
pub struct Cell {
    pub c: char,
    pub bg: u16,
    pub fg: u16,
}

impl From<Indexed<&TermCell>> for Cell {
    fn from(i: Indexed<&TermCell>) -> Self {
        Self {
            c: i.cell.c,
            bg: i.cell.bg.to_u16(),
            fg: i.cell.fg.to_u16(),
        }
    }
}

/// CellsResult contains the pointer to an array of cells and it's length.
#[repr(C)]
pub struct CellsResult {
    cells: *mut Cell,
    length: usize,
}

/// Creates a new `Term` instance with the specified dimensions.
///
/// This function will allocate memory for a new `Term` instance with the provided
/// number of lines and columns. After using the terminal, you should ensure
/// that its memory is deallocated to avoid memory leaks.
///
/// The caller must ensure to deallocate the memory for the `Term` instance
/// when it is no longer needed by converting the raw pointer back into a `Box`
/// and dropping it.
#[no_mangle]
pub extern "C" fn new_term(lines: usize, columns: usize) -> *mut Term {
    Box::into_raw(Box::new(Term::new(lines, columns)))
}

/// Updates the provided `Term` instance and returns the cells that need rendering.
///
/// This function takes in a pointer to a `Term` instance and performs an update
/// on it. It then returns the renderable content as a list of `Cell` objects.
/// The caller is responsible for managing and eventually deallocating the memory
/// of the returned cell list.
///
/// # Safety
///
/// - This function is marked `unsafe` because it dereferences a raw pointer, which
///   can lead to undefined behavior if the pointer is not valid.
/// - The caller must ensure that the provided pointer is valid and points to a `Term`
///   instance previously created with `new_term`.
/// - The caller is also responsible for deallocating the memory of the returned cell list
///   using an appropriate function, like `free_cells`.
#[no_mangle]
pub unsafe extern "C" fn update_term(ptr: *mut Term) -> CellsResult {
    let term = unsafe { &mut *ptr };
    let mut cells: Vec<Cell> = term.renderable_content().map(Into::into).collect();

    let cell_ptr = cells.as_mut_ptr();
    let length = cells.len();
    std::mem::forget(cells);

    CellsResult {
        cells: cell_ptr,
        length,
    }
}

/// Transforms pointer to Term and drops it.
///
/// # Safety
///
/// Raw pointers can be null, misaligned, or dangling, so dereferencing
/// them can cause undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn free_term(ptr: *mut Term) {
    if ptr.is_null() {
        return;
    }
    let _ = Box::from_raw(ptr);
}

/// Free cells returned by the update function.
///
/// The free_cells function is specifically for deallocating the array of Cells
/// that the cells pointer in CellsResult points to. It doesn't deal with the
/// CellsResult struct itself.
///
/// # Safety
///
/// Raw pointers can be null, misaligned, or dangling, so dereferencing
/// them can cause undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn free_cells(ptr: *mut Cell, length: usize) {
    if ptr.is_null() {
        return;
    }
    let _ = Vec::from_raw_parts(ptr, length, length);
}
