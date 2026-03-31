import helper.aleo;
import lib.aleo;
program main.aleo;

function quad:
    input r0 as u32.public;
    call lib.aleo/quadruple r0 into r1;
    output r1 as u32.private;

constructor:
    assert.eq edition 0u16;
