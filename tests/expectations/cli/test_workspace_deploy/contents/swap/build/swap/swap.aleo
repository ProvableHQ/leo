import token.aleo;
program swap.aleo;

function do_swap:
    input r0 as address.private;
    input r1 as u32.private;
    call token.aleo/mint r0 r1 into r2;
    output r2 as u32.private;

constructor:
    assert.eq edition 0u16;
