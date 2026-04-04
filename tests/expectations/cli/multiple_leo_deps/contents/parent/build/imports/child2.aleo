import grandchild.aleo;
program child2.aleo;

record R:
    owner as address.private;
    f2 as field.private;

function main:
    input r0 as u32.private;
    call grandchild.aleo/main r0 into r1;
    output r1 as u32.private;

constructor:
    assert.eq edition 0u16;
