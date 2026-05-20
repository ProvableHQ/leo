import child.aleo;
program parent.aleo;

function main:
    output 42u32 as u32.private;

constructor:
    assert.eq edition 0u16;
