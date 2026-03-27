// --- library: math_lib --- //

// --- Next Module: arith.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn mul(a: u32, b: u32) -> u32 {
    return a * b;
}

// --- Next Program --- //

program submodule_basic.aleo {
    fn main() -> u32 {
        let sum: u32 = math_lib::arith::add(3u32, 4u32);
        let product: u32 = math_lib::arith::mul(3u32, 4u32);
        return math_lib::arith::add(sum, product);
    }
}
