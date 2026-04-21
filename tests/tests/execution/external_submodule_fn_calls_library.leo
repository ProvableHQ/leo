/*
[case]
program = "parent.aleo"
function = "compute"
input = ["5u32"]
*/

// Test: an external Leo program's submodule function itself calls a library
// function.  When `parent.aleo` inlines the submodule fn, the body still
// contains the library call, which must also be inlined on the subsequent pass.
// Exercises the interaction between `is_external_non_library` and `is_library`
// inlining paths.
//
// math_lib::add_one(x)       = x + 1
// child.aleo::util::bumped(x) = math_lib::add_one(x) * 2
// parent.aleo::compute(x)     = child.aleo::util::bumped(x) + math_lib::add_one(x)
// Expected for x = 5:  (5+1)*2 + (5+1) = 12 + 6 = 18

// --- library: math_lib --- //

fn add_one(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Program --- //

program child.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: util.leo --- //

fn bumped(x: u32) -> u32 {
    return math_lib::add_one(x) * 2u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn compute(x: u32) -> u32 {
        let bumped: u32 = child.aleo::util::bumped(x);
        let plain: u32 = math_lib::add_one(x);
        return bumped + plain;
    }

    @noupgrade
    constructor() {}
}
