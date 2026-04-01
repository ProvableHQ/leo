program provider.aleo;

struct Color__F3VqkSOxTDS:
    r as u32;
    g as u32;
    b as u32;

function make_color:
    input r0 as u32.private;
    input r1 as u32.private;
    input r2 as u32.private;
    cast r0 r1 r2 into r3 as Color__F3VqkSOxTDS;
    output r3 as Color__F3VqkSOxTDS.private;

function sum_channels:
    input r0 as Color__F3VqkSOxTDS.private;
    add r0.r r0.g into r1;
    add r1 r0.b into r2;
    output r2 as u32.private;

constructor:
    assert.eq edition 0u16;
