program helper.aleo;

function twice:
    input r0 as u32.public;
    add r0 r0 into r1;
    output r1 as u32.public;

constructor:
    assert.eq edition 0u16;
