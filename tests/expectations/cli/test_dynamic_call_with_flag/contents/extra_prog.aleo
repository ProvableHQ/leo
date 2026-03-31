program extra_prog.aleo;

function dbl:
    input r0 as u64.private;
    add r0 r0 into r1;
    output r1 as u64.private;

constructor:
    assert.eq edition 0u16;
