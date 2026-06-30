// Test: library functions unreachable from the program are dropped, including in submodules, while
// directly- and transitively-called functions are retained. Covers top-level, within-module, and
// cross-module transitive reachability.

// --- library: math_lib --- //

export fn used(a: u32) -> u32 {
    return helper(a);
}

export fn helper(a: u32) -> u32 {
    return deep(a);
}

export fn deep(a: u32) -> u32 {
    return a + 1u32;
}

export fn unused(a: u32) -> u32 {
    return a - 1u32;
}

// --- Next Module: ops.leo --- //
export fn used_mod(a: u32) -> u32 {
    return mod_helper(a);
}

export fn mod_helper(a: u32) -> u32 {
    return math_lib::extra::scale(a);
}

export fn unused_mod(a: u32) -> u32 {
    return a / 2u32;
}

// --- Next Module: extra.leo --- //
export fn scale(a: u32) -> u32 {
    return a * 2u32;
}

export fn unused_extra(a: u32) -> u32 {
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
