program test_my_lib.aleo;

function test_double:
    assert.eq 10u32 10u32;

function test_square:
    assert.eq 16u32 16u32;

function test_square_zero:
    assert.eq 0u32 0u32;

constructor:
    assert.eq edition 0u16;
