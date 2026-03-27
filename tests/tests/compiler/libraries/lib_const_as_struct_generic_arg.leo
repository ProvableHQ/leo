// Test: a library const used directly as a generic argument in a struct definition
// within the same library (not a literal).
//
// Grid::[SIDE] and Grid::[DOUBLE_SIDE] in the Square/BigSquare field types are
// library-const references, not literals. After const propagation they should become
// Grid::[4u32] and Grid::[8u32] respectively. This test checks that const propagation
// visits library composite definitions so that monomorphization receives evaluated
// literals rather than unresolved Path expressions.
//
// Dependency graph:
//   shape_lib  (library: const SIDE=4, DOUBLE_SIDE=8, struct Grid::[N], Square, BigSquare)
//   main.aleo  (program: uses shape_lib::Square and shape_lib::BigSquare)

// --- library: shape_lib --- //

const SIDE: u32 = 4u32;
const DOUBLE_SIDE: u32 = SIDE + SIDE;

struct Grid::[N: u32] {
    cells: [u32; N],
}

struct Square {
    grid: Grid::[SIDE],
}

struct BigSquare {
    grid: Grid::[DOUBLE_SIDE],
}

// --- Next Program --- //

program main.aleo {
    fn get_cell(s: shape_lib::Square) -> u32 {
        return s.grid.cells[0];
    }

    fn get_big_cell(b: shape_lib::BigSquare) -> u32 {
        return b.grid.cells[0];
    }

    @noupgrade
    constructor() {}
}
