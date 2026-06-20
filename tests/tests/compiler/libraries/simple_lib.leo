// --- library: lib2 --- //

export const PP: u32 = 52;
export const XX: u32 = PP;

// --- Next Program --- //

// --- library: lib1 --- //

export const P: u32 = 52 + lib2::PP;
export const X: u32 = P + lib2::XX;

// --- Next Program --- //

export const P: u32 = 52;
export const X: u32 = P;

program simple.aleo {
    fn foo() -> u32 {
        return lib1::X + lib1::P + lib2::XX + lib2::PP + X + P + dep::P + dep::X;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: dep.leo --- //
export const P: u32 = 52;
export const X: u32 = P;
