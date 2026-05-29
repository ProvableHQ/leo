program my_program.aleo;

function greet:
    input r0 as address.private;
    output 1u64 as u64.private;

constructor:
    assert.eq edition 0u16;
