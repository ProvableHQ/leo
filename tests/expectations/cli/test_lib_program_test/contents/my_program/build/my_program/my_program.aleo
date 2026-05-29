program my_program.aleo;

function via_calc:
    input r0 as u32.private;
    mul r0 2u32 into r1;
    mul r1 2u32 into r2;
    output r2 as u32.private;

function via_base:
    input r0 as u32.private;
    mul r0 2u32 into r1;
    output r1 as u32.private;

function via_triple:
    input r0 as u32.private;
    mul r0 3u32 into r1;
    output r1 as u32.private;

function via_fivefold:
    input r0 as u32.private;
    mul r0 5u32 into r1;
    output r1 as u32.private;

constructor:
    assert.eq edition 0u16;
