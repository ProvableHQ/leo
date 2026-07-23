// --- library: geo_lib --- //

// --- Next Module: types.leo --- //
export struct Vec2 {
    x: u32,
    y: u32,
}

export fn magnitude_sq(v: Vec2) -> u32 {
    return v.x * v.x + v.y * v.y;
}

export fn make_vec2(x: u32, y: u32) -> Vec2 {
    return Vec2 { x, y };
}

// --- Next Program --- //

program submodule_struct.aleo {
    fn main() -> u32 {
        let v: geo_lib::types::Vec2 = geo_lib::types::make_vec2(3u32, 4u32);
        return geo_lib::types::magnitude_sq(v);
    }

    @noupgrade
    constructor() {}
}
