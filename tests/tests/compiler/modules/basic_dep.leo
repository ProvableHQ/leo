program parent.aleo {
    fn foo() -> u32 {
        return dep::X + dep::inner::Y;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: dep.leo --- //

export const X: u32 = 32;

// --- Next Module: dep/inner.leo --- //

export const Y: u32 = 32;
