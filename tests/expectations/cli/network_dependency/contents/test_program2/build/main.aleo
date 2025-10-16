import test_program1.aleo;
program test_program2.aleo;

function main:
    call test_program1.aleo/main 2u32 into r0;
    output r0 as u32.private;

constructor:
    assert.eq edition 0u16;
