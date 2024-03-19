/// Definitions in this file are supposed to be use together with libvinilla.h.
///
/// To use vinilla-lib as crate in rust projects, please use vinilla-lib::Processor,
/// vinilla-lib::Term and others directly.
use crate::color::ColorExt;
use crate::processor::Processor;

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::cell::RefCell;

use vinilla_term::{Cell as TermCell, Indexed, Term};
use vte::Parser;

#[repr(C)]
pub struct Cell {
    pub c: u8,
    pub bg: u16,
    pub fg: u16,
}

impl From<Indexed<&TermCell>> for Cell {
    fn from(i: Indexed<&TermCell>) -> Self {
        Self {
            c: i.cell.c as u8,
            bg: i.cell.bg.to_u16(),
            fg: i.cell.fg.to_u16(),
        }
    }
}

/// CellsResult contains the pointer to an array of cells and it's length.
#[repr(C)]
pub struct CellsResult {
    pub cells: *mut Cell,
    pub length: usize,
}

pub struct TermState {
    pub term: Rc<RefCell<Term>>,
    pub state: Parser,
    pub processor: Processor,
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
pub extern "C" fn new_term(lines: usize, columns: usize) -> *mut TermState {
    let term = Rc::new(RefCell::new(Term::new(lines, columns)));
    let state = Parser::new();
    let processor = Processor::new(term.clone());

    Box::into_raw(Box::new(TermState {
        term,
        state,
        processor,
    }))
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
pub unsafe extern "C" fn update_term(
    ptr: *mut TermState,
    bytes: *const u8,
    length: usize,
) -> CellsResult {
    let term_state = unsafe { &mut *ptr };
    let byte_slice = alloc::slice::from_raw_parts(bytes, length);

    for &byte in byte_slice {
        term_state.state.advance(&mut term_state.processor, byte);
    }

    let term = term_state.term.borrow();
    let mut cells: Vec<Cell> = term.renderable_content().map(Into::into).collect();

    let cell_ptr = cells.as_mut_ptr();
    let length = cells.len();
    core::mem::forget(cells);

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
pub unsafe extern "C" fn free_term(ptr: *mut TermState) {
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
