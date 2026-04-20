// --- aleo stub --- //
program base.aleo;

struct Data:
    x as u32;
    y as u32;

function make_data:
    input r0 as u32.private;
    input r1 as u32.private;
    cast r0 r1 into r2 as Data;
    output r2 as Data.private;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import base.aleo;

program middle.aleo {
    fn wrap_data(x: u32, y: u32) -> base.aleo::Data {
        return base.aleo::Data { x: x, y: y };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import base.aleo;
import middle.aleo;

program top.aleo {
    fn main(x: u32, y: u32) -> base.aleo::Data {
        return middle.aleo::wrap_data(x, y);
    }
}
