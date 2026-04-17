import provider.aleo;
program consumer.aleo;

function identity:
    input r0 as provider.aleo/Color__F3VqkSOxTDS.private;
    output r0 as provider.aleo/Color__F3VqkSOxTDS.private;

function make_white:
    cast 255u32 255u32 255u32 into r0 as provider.aleo/Color__F3VqkSOxTDS;
    output r0 as provider.aleo/Color__F3VqkSOxTDS.private;

function sum_of_white:
    cast 255u32 255u32 255u32 into r0 as provider.aleo/Color__F3VqkSOxTDS;
    call provider.aleo/sum_channels r0 into r1;
    output r1 as u32.private;

function average:
    input r0 as provider.aleo/Color__F3VqkSOxTDS.private;
    input r1 as provider.aleo/Color__F3VqkSOxTDS.private;
    add r0.r r1.r into r2;
    div r2 2u32 into r3;
    add r0.g r1.g into r4;
    div r4 2u32 into r5;
    add r0.b r1.b into r6;
    div r6 2u32 into r7;
    cast r3 r5 r7 into r8 as provider.aleo/Color__F3VqkSOxTDS;
    output r8 as provider.aleo/Color__F3VqkSOxTDS.private;

function blend_twice:
    input r0 as provider.aleo/Color__F3VqkSOxTDS.private;
    input r1 as provider.aleo/Color__F3VqkSOxTDS.private;
    input r2 as provider.aleo/Color__F3VqkSOxTDS.private;
    add r0.r r1.r into r3;
    div r3 2u32 into r4;
    add r0.g r1.g into r5;
    div r5 2u32 into r6;
    add r0.b r1.b into r7;
    div r7 2u32 into r8;
    cast r4 r6 r8 into r9 as provider.aleo/Color__F3VqkSOxTDS;
    add r9.r r2.r into r10;
    div r10 2u32 into r11;
    add r9.g r2.g into r12;
    div r12 2u32 into r13;
    add r9.b r2.b into r14;
    div r14 2u32 into r15;
    cast r11 r13 r15 into r16 as provider.aleo/Color__F3VqkSOxTDS;
    output r16 as provider.aleo/Color__F3VqkSOxTDS.private;

function channel_count:
    output 3u32 as u32.private;

constructor:
    assert.eq edition 0u16;
