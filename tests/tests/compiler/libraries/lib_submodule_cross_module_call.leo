// --- library: utils_lib --- //

// --- Next Module: math.leo --- //
fn square(x: u32) -> u32 {
    return x * x;
}

// --- Next Module: geometry.leo --- //
fn distance_sq(ax: u32, ay: u32, bx: u32, by: u32) -> u32 {
    let dx: u32 = bx - ax;
    let dy: u32 = by - ay;
    return utils_lib::math::square(dx) + utils_lib::math::square(dy);
}

// --- Next Program --- //

program cross_module_call.aleo {
    fn main() -> u32 {
        return utils_lib::geometry::distance_sq(0u32, 0u32, 3u32, 4u32);
    }
}
