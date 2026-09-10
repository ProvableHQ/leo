// #28996, the literal case: a dependency uses `Option` internally (here a
// storage value), so its bytecode defines the compiler-generated wrapper
// `Optional__<hash>` (for `u32?`). A consumer importing only the Aleo bytecode
// can construct and inspect that struct by spelling out the full mangled name.
//
// This test also verifies that constructing the imported generated wrapper does
// not make it compiler-generated provenance in the consumer. Reading the
// dependency's `maybe__` mapping across programs (and the rejection of external
// writes) is covered by `mappings/read_external_underscore_mapping.leo` and
// `mappings/modify_external_underscore_mapping_fail.leo`.

// --- aleo stub --- //
program optprod.aleo;

struct Optional__JzunLORyB8U:
    is_some as boolean;
    val as u32;

mapping maybe__:
    key as boolean.public;
    value as u32.public;

function peek:
    async peek into r0;
    output r0 as optprod.aleo/peek.future;

finalize peek:
    contains maybe__[false] into r0;
    get.or_use maybe__[false] 0u32 into r1;
    ternary r0 r1 0u32 into r2;
    cast r0 r2 into r3 as Optional__JzunLORyB8U;
    ternary r3.is_some r3.val 0u32 into r4;
    assert.eq r4 r4;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import optprod.aleo;

program optconsumer.aleo {
    // Construct the dependency's generated `Optional__<hash>` and read its fields.
    fn build(v: u32) -> u32 {
        let o: optprod.aleo::Optional__JzunLORyB8U =
            optprod.aleo::Optional__JzunLORyB8U { is_some: true, val: v };
        return o.is_some ? o.val : 0u32;
    }

    // An imported wrapper with the generated name and layout is not compiler provenance.
    fn build_from_condition(condition: bool, v: u32, fallback: u32) -> u32 {
        let selected: u32 = condition ? v : 0u32;
        let o: optprod.aleo::Optional__JzunLORyB8U =
            optprod.aleo::Optional__JzunLORyB8U { is_some: condition, val: selected };
        let alias: optprod.aleo::Optional__JzunLORyB8U = o;
        return alias.is_some ? alias.val : fallback;
    }

    @noupgrade
    constructor() {}
}
