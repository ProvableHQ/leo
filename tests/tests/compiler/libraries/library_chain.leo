// Test: a chain of 3 libraries, each depending on the previous one.
// Verifies deep transitive library resolution.

// --- library: base --- //

const A: u32 = 1u32;

// --- Next Program --- //

// --- library: middle --- //

const B: u32 = base::A + 2u32;

// --- Next Program --- //

// --- library: top --- //

const C: u32 = middle::B + base::A + 3u32;

// --- Next Program --- //

program chain.aleo {
    // base::A=1, middle::B=3, top::C=7
    fn foo() -> u32 {
        return base::A + middle::B + top::C;
    }

    @noupgrade
    constructor() {}
}
