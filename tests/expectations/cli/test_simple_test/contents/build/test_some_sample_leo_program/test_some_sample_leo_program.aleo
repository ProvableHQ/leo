import some_sample_leo_program.aleo;
program test_some_sample_leo_program.aleo;

function do_nothing:
    call some_sample_leo_program.aleo/main 2u32 3u32 into r0;
    assert.eq r0 3u32;

function test_it:
    call some_sample_leo_program.aleo/main 1u32 2u32 into r0;
    assert.eq r0 3u32;

constructor:
    assert.eq edition 0u16;
