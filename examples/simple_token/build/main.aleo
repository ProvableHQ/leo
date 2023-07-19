program simple_token.aleo;

record Token:
    owner as address.private;
    gates as u64.private;
    amount as u64.private;

closure hello:
    input r0 as u32;
    call world r0 into r1;
    add r1 1u32 into r2;
    output r2 as u32;

closure world:
    input r0 as u32;
    add r0 1u32 into r1;
    output r1 as u32;


function mint:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 0u64 r1 into r2 as Token.record;
    output r2 as Token.record;

function transfer:
    input r0 as Token.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.amount r2 into r3;
    cast r0.owner r0.gates r3 into r4 as Token.record;
    cast r1 0u64 r2 into r5 as Token.record;
    output r4 as Token.record;
    output r5 as Token.record;

