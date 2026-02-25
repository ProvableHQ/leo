import test_json_program.aleo;
program test_main.aleo;

function placeholder:

function test_addition:
    call test_json_program.aleo/main 1u32 2u32 into r0;
    assert.eq r0 3u32;
