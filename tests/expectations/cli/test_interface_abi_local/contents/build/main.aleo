program foo.aleo;

function compute_sum:
    input r0 as u64.private;
    input r1 as u64.private;
    add r0 r1 into r2;
    output r2 as u64.private;

function compute_double:
    input r0 as u64.private;
    add r0 r0 into r1;
    output r1 as u64.private;

constructor:
    assert.eq edition 0u16;
