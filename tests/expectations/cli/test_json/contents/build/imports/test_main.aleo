import test_json_program.aleo;
program test_main.aleo;

function test_addition:
    call test_json_program.aleo/main 1u32 2u32 into r0;
    assert.eq r0 3u32;

constructor:
    assert.eq edition 0u16;
