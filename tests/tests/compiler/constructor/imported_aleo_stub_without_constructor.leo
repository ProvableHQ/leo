// --- aleo stub --- //
program dependency.aleo;

function main:
    assert.eq 1u8 1u8;

// --- Next Program --- //

import dependency.aleo;

program test.aleo {
    fn main() {}

    @noupgrade
    constructor() {}
}
