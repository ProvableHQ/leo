// Test: a library struct (both concrete and generic) in function parameter and return types.
//
// Verifies that library composite types in function signature positions (not just inside
// function bodies) are correctly reconstructed and that monomorphization is triggered
// from function signature scanning, not only from body expressions.
//
// Dependency graph:
//   vec_lib   (library: concrete struct Vec2, generic struct VecN::[N: u32])
//   main.aleo (program: functions with vec_lib types in params and return types)

// --- library: vec_lib --- //

struct Vec2 {
    x: u32,
    y: u32,
}

struct VecN::[N: u32] {
    len: u32,
}

// --- Next Program --- //

program main.aleo {
    fn scale(v: vec_lib::Vec2, factor: u32) -> vec_lib::Vec2 {
        return vec_lib::Vec2 { x: v.x * factor, y: v.y * factor };
    }

    fn make_vec3(v: vec_lib::Vec2) -> vec_lib::VecN::[3u32] {
        return vec_lib::VecN::[3u32] { len: 3u32 };
    }

    fn get_len(v: vec_lib::VecN::[5u32]) -> u32 {
        return v.len;
    }

    @noupgrade
    constructor() {}
}
