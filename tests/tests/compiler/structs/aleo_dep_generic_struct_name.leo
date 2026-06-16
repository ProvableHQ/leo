// #28996: a dependency exposes a monomorphized const-generic struct, whose name
// is the compiler-generated `Slot__<hash>` (from `Slot::[8u32]`), in its
// transition signature. A consumer importing only the Aleo bytecode references
// that type by spelling out the full mangled name.

// --- aleo stub --- //
program genprod.aleo;

struct Slot__LbJovpLEwVd:
    val as u32;

function store:
    input r0 as Slot__LbJovpLEwVd.private;
    output r0.val as u32.private;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import genprod.aleo;

program genconsumer.aleo {
    // Construct the dependency's generic `__` struct directly.
    fn build(v: u32) -> u32 {
        let s: genprod.aleo::Slot__LbJovpLEwVd = genprod.aleo::Slot__LbJovpLEwVd { val: v };
        return s.val;
    }

    // Pass it into the dependency's external call.
    fn via_call(v: u32) -> u32 {
        let s: genprod.aleo::Slot__LbJovpLEwVd = genprod.aleo::Slot__LbJovpLEwVd { val: v };
        return genprod.aleo::store(s);
    }

    @noupgrade
    constructor() {}
}
