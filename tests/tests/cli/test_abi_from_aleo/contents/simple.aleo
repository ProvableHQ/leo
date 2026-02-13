program simple.aleo;

struct Point:
    x as i32;
    y as i32;

record Token:
    owner as address.private;
    amount as u64.public;

mapping balances:
    key as address.public;
    value as u64.public;

function add_numbers:
    input r0 as u32.private;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;

function get_point:
    input r0 as Point.private;
    output r0 as Point.private;

function get_token:
    input r0 as Token.record;
    output r0 as Token.record;
