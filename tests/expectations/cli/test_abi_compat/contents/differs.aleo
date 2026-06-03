program differs.aleo;

struct Metadata:
    decimals as u16;
    supply as u64;

record Token:
    owner as address.private;
    amount as u32.private;

mapping balances:
    key as address.public;
    value as u64.public;

function transfer:
    input r0 as Token.record;
    input r1 as address.private;
    input r2 as u32.public;
    output r0 as Token.record;

function describe:
    input r0 as Metadata.private;
    output r0 as Metadata.private;
