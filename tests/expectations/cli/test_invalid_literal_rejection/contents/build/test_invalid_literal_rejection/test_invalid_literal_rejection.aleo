program test_invalid_literal_rejection.aleo;

function main:
    input r0 as field.public;
    add r0 1field into r1;
    output r1 as field.private;

constructor:
    assert.eq edition 0u16;
