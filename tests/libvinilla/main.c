#include "../../libvinilla.h"
#include <stdio.h>

int main() {
    TermState *term_state = new_term(1, 13);
    if (term_state == NULL) {
        fprintf(stderr, "Failed to create TermState\n");
        return 1;
    }

    uint8_t bytes[] = "Hello, World!";
    uintptr_t length = sizeof(bytes) - 1;
    CellsResult cells_result = update_term(term_state, bytes, length);

    for (uintptr_t i = 0; i < cells_result.length; i++) {
        putchar((int) cells_result.cells[i].c);
    }

    putchar('\n');

    free_cells(cells_result.cells, cells_result.length);
    free_term(term_state);

    return 0;
}
