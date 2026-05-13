program victim.aleo;

function evil:
    input r0 as victim.aleo/evil.future;
    output r0 as u32.public;

constructor:
    assert.eq edition 0u16;
