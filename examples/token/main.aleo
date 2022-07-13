program token.aleo;

record token:
    owner as address.private;
    balance as u64.private;
    token_amount as u64.private;

function compute:
    input r0 as token.record;
    add r0.token_amount r0.token_amount into r1;
    output r1 as u64.private;
