#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Cell {
  uint32_t c;
  uint16_t bg;
  uint16_t fg;
};

/// CellsResult contains the pointer to an array of cells and it's length.
struct CellsResult {
  Cell *cells;
  uintptr_t length;
};

extern "C" {

/// Creates a new `Term` instance with the specified dimensions.
///
/// This function will allocate memory for a new `Term` instance with the provided
/// number of lines and columns. After using the terminal, you should ensure
/// that its memory is deallocated to avoid memory leaks.
///
/// The caller must ensure to deallocate the memory for the `Term` instance
/// when it is no longer needed by converting the raw pointer back into a `Box`
/// and dropping it.
Term *new_term(uintptr_t lines, uintptr_t columns);

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
CellsResult update_term(Term *ptr);

/// Transforms pointer to Term and drops it.
///
/// # Safety
///
/// Raw pointers can be null, misaligned, or dangling, so dereferencing
/// them can cause undefined behavior.
void free_term(Term *ptr);

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
void free_cells(Cell *ptr, uintptr_t length);

} // extern "C"
