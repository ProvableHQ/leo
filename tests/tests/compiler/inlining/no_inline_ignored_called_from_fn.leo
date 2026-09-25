@no_inline
export fn inner(x: u32) -> u32 {
    return x + 2u32;
}

export fn outer(x: u32) -> u32 {
    return inner(x);
}

program child.aleo {
    fn main(x: u32) -> u32 {
        return outer(x);
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child.aleo;

@no_inline
export fn inner(x: u32) -> u32 {
    return x + 1u32;
}

program test.aleo {
    fn main(x: u32) -> (u32, u32) {
        return (inner(x), child.aleo::main(x));
    }

    @noupgrade
    constructor() {}
}
