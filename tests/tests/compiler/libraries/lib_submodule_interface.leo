// Test: interface definitions are allowed inside library modules.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
interface Addable {
    fn add(a: u32, b: u32) -> u32;
}

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program interface_in_module.aleo {
    fn main() -> u32 {
        return math_lib::ops::add(1u32, 2u32);
    }

    @noupgrade
    constructor() {}
}
