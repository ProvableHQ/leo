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

function channel_count:
    output 3u32 as u32.private;

constructor:
    assert.eq edition 0u16;
