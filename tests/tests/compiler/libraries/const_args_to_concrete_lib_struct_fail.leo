// Test: providing const arguments to a non-generic (concrete) library struct.
// Point has no const parameters so Point::[5u32] is invalid.
// Expects a TypeCheckerError for unexpected const args.

// --- library: foo_lib --- //

struct Point {
    x: u32,
    y: u32,
}

// --- Next Program --- //

program main.aleo {
    fn compute() -> u32 {
        let p: foo_lib::Point::[5u32] = foo_lib::Point::[5u32] { x: 1u32, y: 2u32 };
        return p.x;
    }

    @noupgrade
    constructor() {}
}
