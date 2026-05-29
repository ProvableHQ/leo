import my_program.aleo;
program test_my_program.aleo;

function test_via_calc:
    call my_program.aleo/via_calc 3u32 into r0;
    assert.eq r0 12u32;

function test_via_base:
    call my_program.aleo/via_base 5u32 into r0;
    assert.eq r0 10u32;

function test_via_triple:
    call my_program.aleo/via_triple 4u32 into r0;
    assert.eq r0 12u32;

function test_via_fivefold:
    call my_program.aleo/via_fivefold 4u32 into r0;
    assert.eq r0 20u32;

constructor:
    assert.eq edition 0u16;
