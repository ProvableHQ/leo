import multi_test_prog.aleo;
program test_addition.aleo;

function adds_small_numbers:
    call multi_test_prog.aleo/main 1u32 2u32 into r0;
    assert.eq r0 3u32;

constructor:
    assert.eq edition 0u16;
