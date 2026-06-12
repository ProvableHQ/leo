program parent.aleo {
    fn foo() -> u32 {
        return dep::X;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: dep.leo --- //

const X: u32 = 10;

// --- Next Module: program.leo --- //

const Y: u32 = 20;
