program my_app.aleo;

struct Pair__LEvJOzNoBnC:
    a as u32;
    b as u32;

function combine:
    input r0 as u32.public;
    input r1 as u32.public;
    add r0 r1 into r2;
    output r2 as u32.private;

constructor:
    assert.eq edition 0u16;
