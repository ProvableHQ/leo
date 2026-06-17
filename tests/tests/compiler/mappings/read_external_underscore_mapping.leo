// #28996: a dependency's bytecode can expose a compiler-generated mapping whose
// name contains `__` (here `maybe__`, as emitted for an internal `Option` store).
// A consumer importing only the Aleo bytecode can read from that mapping by
// spelling out the full name — the permitted external operations `contains`,
// `get`, and `get_or_use` all work regardless of the `__` in the name. External
// writes remain illegal; see `modify_external_underscore_mapping_fail.leo`.

// --- aleo stub --- //
program optprod.aleo;

mapping maybe__:
    key as boolean.public;
    value as u32.public;

function seed:
    async seed into r0;
    output r0 as optprod.aleo/seed.future;

finalize seed:
    set 1u32 into maybe__[true];

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import optprod.aleo;

program optconsumer.aleo {
    fn peek() -> Final {
        return final { finalize_peek(); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_peek() {
    let present: bool = Mapping::contains(optprod.aleo::maybe__, true);
    let v: u32 = Mapping::get(optprod.aleo::maybe__, true);
    let w: u32 = Mapping::get_or_use(optprod.aleo::maybe__, false, 0u32);
    assert_eq(present, present);
    assert_eq(v, v);
    assert_eq(w, w);
}
