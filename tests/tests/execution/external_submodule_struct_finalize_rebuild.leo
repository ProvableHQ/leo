/*
[case]
program = "parent.aleo"
function = "run"
input = ["{ hi: 0u128, lo: 7u128 }"]
*/

// Regression test for #29586. In a finalize, a struct from a cross-program
// submodule is rebuilt from the members of an existing external-struct register
// (`cast x.hi x.lo into r as child.aleo/S`). snarkVM validates a cast to an
// external struct against the external program's stack and then resolves the
// operand member accesses there too, hitting "main program as external". Codegen
// must not emit a struct that merely reconstructs an external struct from its own
// members; the reconstruction is an identity and is elided.

program child.aleo {
    @noupgrade
    constructor() {}
    fn noop() {}
}

// --- Next Module: m.leo --- //

export struct S {
    hi: u128,
    lo: u128,
}

// Inlined into the caller's finalize; rebuilds `x` from its own members.
export fn rebuild(x: S) -> S {
    return S { hi: x.hi, lo: x.lo };
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    @noupgrade
    constructor() {}

    fn run(x: child.aleo::m::S) -> Final {
        return final {
            let r: child.aleo::m::S = child.aleo::m::rebuild(x);
            assert_eq(r.lo, x.lo);
        };
    }
}
