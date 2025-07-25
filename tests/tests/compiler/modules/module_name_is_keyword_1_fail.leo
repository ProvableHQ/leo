program parent.aleo {
    transition foo() -> u32 {
        return dep::X;
    }
}

// --- Next Module: dep.leo --- //

const X: u32 = 10;

// --- Next Module: transition.leo --- //

const Y: u32 = 20;
