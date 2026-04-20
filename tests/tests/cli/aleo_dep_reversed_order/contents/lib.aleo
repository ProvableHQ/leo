import helper.aleo;

program lib.aleo;

function quadruple:
    input r0 as u32.public;
    call helper.aleo/twice r0 into r1;
    call helper.aleo/twice r1 into r2;
    output r2 as u32.public;

constructor:
    assert.eq edition 0u16;
