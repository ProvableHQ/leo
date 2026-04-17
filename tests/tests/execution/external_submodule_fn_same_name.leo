/*
[case]
program = "parent.aleo"
function = "call_both"
input = ["5u32"]
*/

// Test: two distinct external Leo programs define submodule functions with the
// same path (`math::helper`).  The inliner must key functions by the full
// `Location` (program + path) rather than path alone — otherwise one of the two
// helpers would shadow the other and both call sites would be inlined with the
// same body.
//
// A.aleo::math::helper(x) = x + 100
// B.aleo::math::helper(x) = x * 10
// parent.aleo::call_both(x) = A.aleo::math::helper(x) + B.aleo::math::helper(x)
// Expected for x = 5:  (5 + 100) + (5 * 10) = 105 + 50 = 155

program a.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

fn helper(x: u32) -> u32 {
    return x + 100u32;
}

// --- Next Program --- //

program b.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

fn helper(x: u32) -> u32 {
    return x * 10u32;
}

// --- Next Program --- //

import a.aleo;
import b.aleo;

program parent.aleo {
    fn call_both(x: u32) -> u32 {
        let a: u32 = a.aleo::math::helper(x);
        let b: u32 = b.aleo::math::helper(x);
        return a + b;
    }

    @noupgrade
    constructor() {}
}
