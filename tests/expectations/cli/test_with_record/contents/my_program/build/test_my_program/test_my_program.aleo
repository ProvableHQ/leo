import my_program.aleo;
program test_my_program.aleo;

function test_double_amount:
    call my_program.aleo/double_amount 5u64 into r0;
    assert.eq r0 10u64;
    call my_program.aleo/double_amount 0u64 into r1;
    assert.eq r1 0u64;

constructor:
    assert.eq edition 0u16;
