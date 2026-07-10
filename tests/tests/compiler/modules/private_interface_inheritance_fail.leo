// Test: an interface in one module cannot inherit from a non-`export` interface
// declared in a different module of the same program.

program inherit_test.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: parent.leo --- //

export interface I : helpers::J {
    fn foo();
}

// --- Next Module: parent/helpers.leo --- //

interface J {
    fn foo();
}
