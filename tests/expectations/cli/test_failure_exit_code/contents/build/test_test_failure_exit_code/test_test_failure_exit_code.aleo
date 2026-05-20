import test_failure_exit_code.aleo;
program test_test_failure_exit_code.aleo;

function test_deliberate_failure:
    assert.eq 1u8 2u8;

constructor:
    assert.eq edition 0u16;
