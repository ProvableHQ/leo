program test_my_lib.aleo;

function test_double:
    assert.eq 10u32 10u32;

function test_double_zero:
    assert.eq 0u32 0u32;

function test_triple:
    assert.eq 12u32 12u32;

function test_triple_zero:
    assert.eq 0u32 0u32;

constructor:
    assert.eq edition 0u16;
