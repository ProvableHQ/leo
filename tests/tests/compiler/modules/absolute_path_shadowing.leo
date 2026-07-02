program absolute_path_shadowing.aleo {
    fn main() -> u32 {
        return a::both();
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: a.leo --- //

// `c::f` resolves to the nested module `a/c`, while `::c::f` resolves to the
// root module `c`.
export fn both() -> u32 {
    return c::f() + ::c::f();
}

// --- Next Module: a/c.leo --- //

export fn f() -> u32 {
    return 1u32;
}

// --- Next Module: c.leo --- //

export fn f() -> u32 {
    return 100u32;
}
