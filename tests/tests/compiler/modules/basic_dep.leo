program parent.aleo {
    fn foo() -> u32 {
        return dep::X + dep::inner::Y;
    }
}

// --- Next Module: dep.leo --- //

const X: u32 = 32;

// --- Next Module: dep/inner.leo --- //

const Y: u32 = 32;
