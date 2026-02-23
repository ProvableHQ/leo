interface Counter {
    fn increment() -> u64;
}

program lib.aleo {
    fn dummy() {}
}

// --- Next Program --- //

import lib.aleo;

program test.aleo : lib.aleo/Counter {
    fn main(x: u64) -> u64 {
        return 0;
    }
}
