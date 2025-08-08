program parent.aleo {
    transition foo() -> u32 {
        return dep::X + dep::inner::Y;
    }
}

// --- Next Module: dep --- //

const X: u32 = 32;

// --- Next Module: dep/inner --- //

const Y: u32 = 32;
