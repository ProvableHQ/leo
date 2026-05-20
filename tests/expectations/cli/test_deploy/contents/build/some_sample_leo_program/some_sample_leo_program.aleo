program some_sample_leo_program.aleo;

record Token:
    owner as address.private;
    amount as u64.private;

function main:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;

function mint:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 r1 into r2 as Token.record;
    output r2 as Token.record;

function dynamic_transfer:
    input r0 as field.private;
    input r1 as dynamic.record;
    input r2 as address.private;
    call.dynamic r0 'aleo' 'transfer' with r1 r2 (as dynamic.record address.private) into r3 (as dynamic.record);
    output r3 as dynamic.record;

constructor:
    assert.eq edition 0u16;
