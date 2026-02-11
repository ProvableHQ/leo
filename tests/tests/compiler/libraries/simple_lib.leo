// --- library: lib2 --- //

const PP: u32 = 52;
const XX: u32 = PP;


// --- Next Program --- //

// --- library: lib1 --- //

const P: u32 = 52 + lib2::PP;
const X: u32 = P + lib2::XX;

// --- Next Program --- //

const P: u32 = 52;
const X: u32 = P;

program simple.aleo {
    fn foo() -> u32 {
        return lib1::X + lib1::P + lib2::XX + lib2::PP + X + P + dep::P + dep::X;
    }
}

// --- Next Module: dep.leo --- //
const P: u32 = 52;
const X: u32 = P;
