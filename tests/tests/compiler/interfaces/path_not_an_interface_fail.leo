struct NotAnInterface {
    x: u64,
}

program lib.aleo {
    fn dummy(n: NotAnInterface) {}
}

// --- Next Program --- //

import lib.aleo;

program test.aleo : lib.aleo/NotAnInterface + u64 {
    fn main(x: u64) -> u64 {
        return 0;
    }
}
