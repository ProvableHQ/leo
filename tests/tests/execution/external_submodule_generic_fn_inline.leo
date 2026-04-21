/*
[case]
program = "parent.aleo"
function = "scale_once"
input = ["6u32"]

[case]
program = "parent.aleo"
function = "scale_twice"
input = ["3u32"]
*/

// Test: a generic submodule fn from an external Leo program is inlined at every call site,
// including when called more than once with different const arguments from the same consumer.
//
// Submodule functions do not appear in Aleo bytecode, so the compiler must always inline them.
// Monomorphization still runs cross-program (specializing `scale::[3u32]`, `scale::[4u32]`),
// but the specialized instances are then force-inlined because their paths still have length > 1.
//
// scale_once  — single call site:  scale::[3u32](6) = 6 * 3 = 18
// scale_twice — two call sites:    scale::[4u32](3) + scale::[4u32](3) = (3*4) + (3*4) = 24

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
    // Expected: scale::[3u32](6) = 6 * 3 = 18
    fn scale_once(x: u32) -> u32 {
        return child.aleo::math::scale::[3u32](x);
    }

    // scale::[4u32] is called at two independent call sites — both must be inlined.
    // Expected: scale::[4u32](3) + scale::[4u32](3) = (3 * 4) + (3 * 4) = 24
    fn scale_twice(x: u32) -> u32 {
        let a: u32 = child.aleo::math::scale::[4u32](x);
        let b: u32 = child.aleo::math::scale::[4u32](x);
        return a + b;
    }

    @noupgrade
    constructor() {}
}
