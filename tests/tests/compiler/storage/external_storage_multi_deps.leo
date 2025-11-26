program base.aleo {
    storage val_u32: u32;

    async transition init() -> Future {
        return async { val_u32 = 99u32; };
    }

    async transition wipe() -> Future {
        return async { val_u32 = none; };
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import base.aleo;

program mid1.aleo {
    storage flag: bool;

    async transition init() -> Future {
        return async {
            flag = true;
            assert(base.aleo/val_u32.unwrap() == 99u32);
        };
    }

    async transition wipe() -> Future {
        return async {
            flag = none;
            assert(base.aleo/val_u32 == none);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import base.aleo;
import mid1.aleo;

program mid2.aleo {
    storage counter: u8;

    async transition init() -> Future {
        return async {
            counter = 7u8;

            assert(base.aleo/val_u32.unwrap() == 99u32);
            assert(mid1.aleo/flag.unwrap() == true);
        };
    }

    async transition wipe() -> Future {
        return async {
            counter = none;

            assert(mid1.aleo/flag == none);
        };
    }

    @noupgrade
    async constructor() {}
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
    async transition test_initialized() -> Future {
        return async {
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
    async transition test_fallback() -> Future {
        return async {
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
    async transition test_none() -> Future {
        return async {
            assert(base.aleo/val_u32 == none);
            assert(mid1.aleo/flag == none);
            assert(mid2.aleo/counter == none);
        };
    }

    @noupgrade
    async constructor() {}
}

