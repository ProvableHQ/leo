/*
[case]
program = "parent.aleo"
function = "use_pair"
input = ["5u32"]
*/

// Test: generic composite (struct Pair::[N: u32]) defined in an external submodule,
// constructed by a generic submodule function, and used by the consumer.
// Verifies that monomorphization handles both generic functions AND generic composites
// from external non-library submodules.
//
// make_pair::[3u32](5) = Pair::[3u32] { a: 5, b: 5 * 3 } = Pair { a: 5, b: 15 }
// use_pair(5) = 5 + 15 = 20
// Expected: 20

program child.aleo {
    fn identity(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

struct Pair::[N: u32] {
    a: u32,
    b: u32,
}

fn make_pair::[N: u32](x: u32) -> Pair::[N] {
    return Pair::[N] { a: x, b: x * N };
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn use_pair(x: u32) -> u32 {
        let p: child.aleo::math::Pair::[3u32] = child.aleo::math::make_pair::[3u32](x);
        return p.a + p.b;
    }

    @noupgrade
    constructor() {}
}
