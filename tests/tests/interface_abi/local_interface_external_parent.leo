// Test: a locally-defined interface extending an external interface causes the
// external parent's ABI to be emitted alongside the local one.

interface Base {
    fn get() -> u64;
}

program lib.aleo {
    @noupgrade
    constructor() {}

    fn unused() {}
}

// --- Next Program --- //
import lib.aleo;

interface Extended: lib.aleo::Base {
    fn set(v: u64) -> u64;
}

program test.aleo: Extended {
    @noupgrade
    constructor() {}

    fn get() -> u64 {
        return 0u64;
    }

    fn set(v: u64) -> u64 {
        return v;
    }
}
