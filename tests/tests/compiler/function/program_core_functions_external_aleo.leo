// --- aleo stub --- //
program child.aleo;

mapping vals:
    key as u32.public;
    value as u32.public;

function entry:
    input r0 as u32.private;
    output r0 as u32.private;

view peek:
    input r0 as u32.public;
    get.or_use vals[r0] 0u32 into r1;
    output r1 as u32.public;

constructor:
    assert.eq edition 0u16;

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    final fn foo() {
        let i: [u8; 32] = std::prog::function_checksum::[child.aleo, 'entry']();
        let j: [u8; 32] = std::prog::function_checksum::[child.aleo, 'peek']();
        assert_neq(i, j);
    }

    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}
