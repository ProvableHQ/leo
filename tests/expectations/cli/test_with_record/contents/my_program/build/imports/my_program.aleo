program my_program.aleo;

record Token:
    owner as address.private;
    amount as u64.private;

function double_amount:
    input r0 as u64.private;
    mul r0 2u64 into r1;
    output r1 as u64.private;

function mint:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 r1 into r2 as Token.record;
    output r2 as Token.record;

constructor:
    assert.eq edition 0u16;
