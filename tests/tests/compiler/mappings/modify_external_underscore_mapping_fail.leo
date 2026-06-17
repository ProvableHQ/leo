// #28996: external mapping writes are rejected even when the dependency's
// mapping name contains `__` (here `maybe__`). The `__` in the name does not
// grant any extra cross-program access — only `contains`, `get`, and
// `get_or_use` are allowed on external mappings; see
// `read_external_underscore_mapping.leo`.

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
    fn poke() -> Final {
        return final {
            //
            // Both writes must be rejected — external mapping mutation is illegal.
            //
            optprod.aleo::maybe__.set(false, 7u32); // ❌ illegal external mutation
            optprod.aleo::maybe__.remove(true); // ❌ illegal external mutation
        };
    }

    @noupgrade
    constructor() {}
}
