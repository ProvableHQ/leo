program absolute_path_unknown.aleo {
    fn main() -> u32 {
        return a::go();
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: a.leo --- //

export fn go() -> u32 {
    return ::nope::thing();
}
