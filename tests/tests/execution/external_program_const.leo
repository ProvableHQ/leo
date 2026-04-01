/*
[case]
program = "parent.aleo"
function = "from_main"
input = []

[case]
program = "parent.aleo"
function = "from_sub"
input = []

[case]
program = "parent.aleo"
function = "combined"
input = []
*/

// Test: accessing constants from an imported .aleo program at runtime.
// child.aleo::MAIN_CONST = 42, child.aleo::sub::SUB_CONST = 10.

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
        return child.aleo::MAIN_CONST;        // 42
    }

    fn from_sub() -> u32 {
        return child.aleo::sub::SUB_CONST;    // 10
    }

    fn combined() -> u32 {
        return child.aleo::MAIN_CONST + child.aleo::sub::SUB_CONST;  // 52
    }

    @noupgrade
    constructor() {}
}
