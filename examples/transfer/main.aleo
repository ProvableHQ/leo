program transfer.aleo;
record token:
    owner as address.private;
    balance as u64.private;
    amount as u64.private;

function main:
    input r0 as address.private;
    input r1 as u64.private;
    token r0 0u64 r1 into r2;
    output r2 as token.record;

function transfer:
    input r0 as token.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.amount r2 into r3;
    token r1 0u64 r2 into r4;
    token r0.owner r0.balance r3 into r5;
    output r4 as token.record;
    output r5 as token.record;
