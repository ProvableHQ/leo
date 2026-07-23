import my_app.aleo;
program test_my_app.aleo;

struct Pair__LEvJOzNoBnC:
    a as u32;
    b as u32;

function test_combine:
    call my_app.aleo/combine 3u32 4u32 into r0;
    assert.eq r0 7u32;

constructor:
    assert.eq edition 0u16;
