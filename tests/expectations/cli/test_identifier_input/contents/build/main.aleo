program test_identifier_input.aleo;

function main:
    input r0 as identifier.private;
    output r0 as identifier.private;

constructor:
    assert.eq edition 0u16;
