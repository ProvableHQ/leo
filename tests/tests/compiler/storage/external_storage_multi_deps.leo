program base.aleo {
    storage val_u32: u32;

    fn init() -> Final {
        return final { val_u32 = 99u32; };
    }

    fn wipe() -> Final {
        return final { val_u32 = none; };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import base.aleo;

program mid1.aleo {
    storage flag: bool;

    fn init() -> Final {
        return final {
            flag = true;
            assert(base.aleo/val_u32.unwrap() == 99u32);
        };
    }

    fn wipe() -> Final {
        return final {
            flag = none;
            assert(base.aleo/val_u32 == none);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import base.aleo;
import mid1.aleo;

program mid2.aleo {
    storage counter: u8;

    fn init() -> Final {
        return final {
            counter = 7u8;

            assert(base.aleo/val_u32.unwrap() == 99u32);
            assert(mid1.aleo/flag.unwrap() == true);
        };
    }

    fn wipe() -> Final {
        return final {
            counter = none;

            assert(mid1.aleo/flag == none);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import base.aleo;
import mid1.aleo;
import mid2.aleo;

program top.aleo {

    //
    // ─────────────────────────────────────────────
    // Check all external values are initialized
    // ─────────────────────────────────────────────
    //
    fn test_initialized() -> Final {
        return final {
            assert(base.aleo/val_u32.unwrap() == 99u32);
            assert(mid1.aleo/flag.unwrap() == true);
            assert(mid2.aleo/counter.unwrap() == 7u8);
        };
    }

    //
    // ─────────────────────────────────────────────
    // Check fallback values after wipe
    // ─────────────────────────────────────────────
    //
    fn test_fallback() -> Final {
        return final {
            assert(base.aleo/val_u32.unwrap_or(0u32) == 0u32);
            assert(mid1.aleo/flag.unwrap_or(false) == false);
            assert(mid2.aleo/counter.unwrap_or(11u8) == 11u8);
        };
    }

    //
    // ─────────────────────────────────────────────
    // None-checks
    // ─────────────────────────────────────────────
    //
    fn test_none() -> Final {
        return final {
            assert(base.aleo/val_u32 == none);
            assert(mid1.aleo/flag == none);
            assert(mid2.aleo/counter == none);
        };
    }

    @noupgrade
    constructor() {}
}

