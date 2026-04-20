// Test: dynamic dispatch through a library-defined interface (cross-library inheritance).

// --- library: base_lib --- //

interface Base {
    fn fetch() -> u32;
}

// --- Next Program --- //

// --- library: ext_lib --- //

interface Extended : base_lib::Base {
    fn store(v: u32) -> u32;
}

// --- Next Program --- //

program impl_cross.aleo : ext_lib::Extended {
    fn fetch() -> u32 {
        return 42u32;
    }

    fn store(v: u32) -> u32 {
        return v + 1u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import impl_cross.aleo;

program caller_cross.aleo {
    fn call_store(target: field, v: u32) -> u32 {
        return ext_lib::Extended@(target)::store(v);
    }

    fn call_fetch(target: field) -> u32 {
        return base_lib::Base@(target)::fetch();
    }

    @noupgrade
    constructor() {}
}
