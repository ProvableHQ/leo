program absolute_path_private.aleo {
    fn main() -> u32 {
        return a::go();
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: a.leo --- //

export fn go() -> u32 {
    return ::c::hidden();
}

// --- Next Module: c.leo --- //

fn hidden() -> u32 {
    return 1u32;
}
