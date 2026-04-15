// Test: accessing constants from an imported .aleo program.
// Covers two cases:
//   1. Top-level const:  child.aleo::MAIN_CONST (single segment)
//   2. Submodule const:  child.aleo::sub::SUB_CONST (multi-segment, new syntax)

program child.aleo {
    const MAIN_CONST: u32 = 42u32;

    fn get_main_const() -> u32 {
        return MAIN_CONST;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: sub.leo --- //

const SUB_CONST: u32 = 10u32;

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn from_main() -> u32 {
        return child.aleo::MAIN_CONST;
    }

    fn from_sub() -> u32 {
        return child.aleo::sub::SUB_CONST;
    }

    fn combined() -> u32 {
        return child.aleo::MAIN_CONST + child.aleo::sub::SUB_CONST;
    }

    @noupgrade
    constructor() {}
}
