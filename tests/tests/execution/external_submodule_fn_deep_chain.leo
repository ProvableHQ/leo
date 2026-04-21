/*
[case]
program = "top.aleo"
function = "pipeline"
input = ["10u32"]
*/

// Test: 3-level deep program chain where the consumer calls submodule functions
// at multiple levels.  The top.aleo program reaches directly into both
// grandchild.aleo's and child.aleo's submodules.  Verifies that the inliner's
// `is_stub_scope` save/restore handles nested FromLeo stubs correctly.
//
// grandchild.aleo::ops::inc(x) = x + 1
// child.aleo::ops::double(x)   = x * 2
// top.aleo::pipeline(x)        = child.aleo::ops::double(grandchild.aleo::ops::inc(x))
//                               = (10 + 1) * 2 = 22
// Expected: 22

program grandchild.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: ops.leo --- //

fn inc(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Program --- //

import grandchild.aleo;

program child.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: ops.leo --- //

fn double(x: u32) -> u32 {
    return x * 2u32;
}

// --- Next Program --- //

import grandchild.aleo;
import child.aleo;

program top.aleo {
    // Calls submodule fns from two different external programs.
    fn pipeline(x: u32) -> u32 {
        let incremented: u32 = grandchild.aleo::ops::inc(x);
        let doubled: u32 = child.aleo::ops::double(incremented);
        return doubled;
    }

    @noupgrade
    constructor() {}
}
