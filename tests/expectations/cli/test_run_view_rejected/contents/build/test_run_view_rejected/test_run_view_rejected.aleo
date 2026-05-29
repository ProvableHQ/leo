program test_run_view_rejected.aleo;

mapping balances:
    key as address.public;
    value as u64.public;

function noop:
    input r0 as u32.private;
    output r0 as u32.private;

view get_balance:
    input r0 as address.public;
    get.or_use balances[r0] 0u64 into r1;
    output r1 as u64.public;

constructor:
    assert.eq edition 0u16;
