// #28996: a dependency can expose a compiler-generated struct name containing
// `__` (here a library composite `geo::Point` lowered to `Point__<hash>` in
// `producer.aleo`'s transition signature). When only the Aleo bytecode is
// available, a consumer can still use that type by spelling out the full
// mangled name — no source and no disassembly desugaring required.

// --- aleo stub --- //
program producer.aleo;

struct Point__IHPtDcGjLAc:
    x as u32;
    y as u32;

function origin:
    input r0 as u32.private;
    input r1 as u32.private;
    cast r0 r1 into r2 as Point__IHPtDcGjLAc;
    output r2 as Point__IHPtDcGjLAc.private;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import producer.aleo;

program consumer.aleo {
    // Construct the dependency's `__` struct and read its fields.
    fn build(x: u32, y: u32) -> u32 {
        let p: producer.aleo::Point__IHPtDcGjLAc = producer.aleo::Point__IHPtDcGjLAc { x: x, y: y };
        return p.x + p.y;
    }

    // Receive the `__` struct as the output of an external call.
    fn via_call(x: u32, y: u32) -> u32 {
        let p: producer.aleo::Point__IHPtDcGjLAc = producer.aleo::origin(x, y);
        return p.x + p.y;
    }

    @noupgrade
    constructor() {}
}
