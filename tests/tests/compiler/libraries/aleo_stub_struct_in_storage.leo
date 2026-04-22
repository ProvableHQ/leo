// Test: a struct from an imported Aleo program is used as a storage type in the
// main program, with `.unwrap()` forcing zero-value synthesis.
//
// Exercises phase-1 `collect_composites_from_aleo_program`: the composite
// `base.aleo::Data` must be registered in `self.composites` (keyed by the Aleo
// program name) so that `wrap_none`'s zero-synthesis lookup succeeds — there
// is no alternative symbol-table fallback for Aleo-stub composites in this
// pass.

// --- aleo stub --- //
program base.aleo;

struct Data:
    x as u32;
    y as u32;

function make_data:
    input r0 as u32.private;
    input r1 as u32.private;
    cast r0 r1 into r2 as Data;
    output r2 as Data.private;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import base.aleo;

program wrapper.aleo {
    storage latest: base.aleo::Data;

    fn peek() -> Final {
        return final {
            let d = latest.unwrap();
            assert_eq(d.x, d.x);
        };
    }

    @noupgrade
    constructor() {}
}
