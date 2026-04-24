/*
[case]
program = "parent.aleo"
function = "compute"
input = ["10u32"]
*/

// Test: the same generic submodule function is called with DIFFERENT const
// arguments from the same consumer function.  Each call site produces a
// distinct monomorphized body that must be inlined independently.
//
// child.aleo::math::scale::[3u32](10) = 10 * 3 = 30
// child.aleo::math::scale::[5u32](10) = 10 * 5 = 50
// Expected: 30 + 50 = 80

program child.aleo {
    fn identity(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn compute(x: u32) -> u32 {
        let a: u32 = child.aleo::math::scale::[3u32](x);
        let b: u32 = child.aleo::math::scale::[5u32](x);
        return a + b;
    }

    @noupgrade
    constructor() {}
}
