/*
[case]
program = "parent.aleo"
function = "call_once"
input = ["5u32"]

[case]
program = "parent.aleo"
function = "call_twice"
input = ["3u32", "4u32"]

[case]
program = "parent.aleo"
function = "call_chained"
input = ["6u32"]
*/

// Test: submodule helper functions from an imported .aleo program are correctly
// inlined at the call site.
//
// Three call-count cases:
//   call_once   — helper called exactly once (simplest inline)
//   call_twice  — helper called twice in the same function (both sites inlined)
//   call_chained — consumer calls a submodule helper that itself calls another
//                  submodule helper (transitive inline)
//
// The `unused_helper` function is defined in the submodule but never called by
// parent.aleo — it exists to verify that dead external submodule code does not
// cause a compiler panic.

program child.aleo {
    fn entry(x: u32) -> u32 {
        return math::double(x);
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

// Called once, twice, and transitively from parent.aleo.
fn double(x: u32) -> u32 {
    return x * 2u32;
}

// Called by triple, which is called by parent — tests transitive inline.
fn add_one(x: u32) -> u32 {
    return x + 1u32;
}

// Calls another submodule helper — tests that chained submodule inlining works.
fn triple(x: u32) -> u32 {
    return add_one(add_one(add_one(x - 1u32)));
}

// Never called by parent — tests that dead external submodule code does not panic.
fn unused_helper(x: u32) -> u32 {
    return x + 99u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // double is inlined once.
    // Expected: 5 * 2 = 10
    fn call_once(x: u32) -> u32 {
        return child.aleo::math::double(x);
    }

    // double is inlined at two call sites.
    // Expected: (3 * 2) + (4 * 2) = 6 + 8 = 14
    fn call_twice(x: u32, y: u32) -> u32 {
        let a: u32 = child.aleo::math::double(x);
        let b: u32 = child.aleo::math::double(y);
        return a + b;
    }

    // triple calls add_one internally — both are inlined transitively.
    // triple(6) = add_one(add_one(add_one(5))) = 8
    fn call_chained(x: u32) -> u32 {
        return child.aleo::math::triple(x);
    }

    @noupgrade
    constructor() {}
}
