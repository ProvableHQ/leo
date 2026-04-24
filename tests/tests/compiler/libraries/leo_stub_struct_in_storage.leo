// Test: one Leo program imports another Leo program, and uses a struct from
// the imported program as a storage type with `.unwrap()` forcing zero-value
// synthesis.
//
// Exercises phase-1 `collect_composites_from_stub` on the `Stub::FromLeo`
// branch: the composite `base.aleo::Data` is defined in a Leo program (not a
// library, not an Aleo bytecode stub), and must be registered in
// `self.composites` via the `FromLeo` match arm so that `wrap_none`'s
// zero-synthesis lookup succeeds.

program base.aleo {
    struct Data {
        x: u32,
        y: u32,
    }

    fn make_data(x: u32, y: u32) -> Data {
        return Data { x: x, y: y };
    }

    @noupgrade
    constructor() {}
}

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
