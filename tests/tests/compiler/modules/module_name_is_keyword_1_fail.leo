program parent.aleo {
    fn foo() -> u32 {
        return dep::X;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: dep.leo --- //

export const X: u32 = 10;

// --- Next Module: program.leo --- //

export const Y: u32 = 20;
