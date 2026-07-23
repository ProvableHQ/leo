// Test: library function creates and uses a library struct as a local variable.

// --- library: geo_lib --- //

export struct Vec2 {
    x: u32,
    y: u32,
}

export fn make_vec(x: u32, y: u32) -> Vec2 {
    let v: Vec2 = Vec2 { x, y };
    return v;
}

export fn sum_components(x: u32, y: u32) -> u32 {
    let v: Vec2 = make_vec(x, y);
    return v.x + v.y;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> u32 {
        return geo_lib::sum_components(a, b);
    }

    @noupgrade
    constructor() {}
}
