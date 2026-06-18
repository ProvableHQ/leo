// Test: library functions unreachable from the program are dropped, including in submodules, while
// directly- and transitively-called functions are retained. Covers top-level, within-module, and
// cross-module transitive reachability.

// --- library: math_lib --- //

fn used(a: u32) -> u32 {
    return helper(a);
}

fn helper(a: u32) -> u32 {
    return deep(a);
}

fn deep(a: u32) -> u32 {
    return a + 1u32;
}

fn unused(a: u32) -> u32 {
    return a - 1u32;
}

// --- Next Module: ops.leo --- //
fn used_mod(a: u32) -> u32 {
    return mod_helper(a);
}

fn mod_helper(a: u32) -> u32 {
    return math_lib::extra::scale(a);
}

fn unused_mod(a: u32) -> u32 {
    return a / 2u32;
}

// --- Next Module: extra.leo --- //
fn scale(a: u32) -> u32 {
    return a * 2u32;
}

fn unused_extra(a: u32) -> u32 {
    return a;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32) -> u32 {
        let a: u32 = math_lib::used(x);
        let b: u32 = math_lib::ops::used_mod(a);
        return b;
    }

    @noupgrade
    constructor() {}
}
