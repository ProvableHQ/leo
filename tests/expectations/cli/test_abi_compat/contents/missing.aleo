program missing.aleo;

record Token:
    owner as address.private;
    amount as u64.private;

function transfer:
    input r0 as Token.record;
    input r1 as address.private;
    input r2 as u64.public;
    output r0 as Token.record;
