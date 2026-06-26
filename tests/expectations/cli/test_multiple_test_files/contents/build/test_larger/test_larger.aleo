import multi_test_prog.aleo;
program test_larger.aleo;

function adds_larger_numbers:
    call multi_test_prog.aleo/main 10u32 5u32 into r0;
    assert.eq r0 15u32;

function wrong_expectation:
    call multi_test_prog.aleo/main 2u32 2u32 into r0;
    assert.eq r0 5u32;

constructor:
    assert.eq edition 0u16;
