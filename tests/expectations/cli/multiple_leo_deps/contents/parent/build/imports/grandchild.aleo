program grandchild.aleo;

record R:
    owner as address.private;

function main:
    input r0 as u32.private;
    output r0 as u32.private;

constructor:
    assert.eq edition 0u16;
