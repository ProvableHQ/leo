interface Base {
    fn get() -> u64;
}

program lib.aleo {
    fn unused() {}
}

// --- Next Program --- //

import lib.aleo;

interface Extended : lib.aleo/Base {
    fn set(v: u64) -> u64;
}

program test.aleo : Extended {
    fn get() -> u64 {
        return 0u64;
    }

    fn set(v: u64) -> u64 {
        return v;
    }
}
