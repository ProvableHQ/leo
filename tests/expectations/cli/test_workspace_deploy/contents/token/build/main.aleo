program token.aleo;

function mint:
    input r0 as address.private;
    input r1 as u32.private;
    output r1 as u32.private;

constructor:
    assert.eq edition 0u16;
