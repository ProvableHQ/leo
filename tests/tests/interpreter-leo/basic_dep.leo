program test.aleo {
    transition main() -> u32 {
        return dep::X + dep::inner::Y 
        + dep::foo(dep::X, dep::inner::Y)
        + dep::inner::foo(dep::X, dep::inner::Y);
    }
}

// --- Next Module: dep.leo --- //

const X: u32 = 32;

inline foo(x: u32, y: u32) -> u32 {
    return x + y; 
}

// --- Next Module: dep/inner.leo --- //

const Y: u32 = 64;

inline foo(x: u32, y: u32) -> u32 {
    return x * y; 
}
