program lib.aleo {
    fn dummy() {}
}

// --- Next Program --- //

import lib.aleo;

program test.aleo : lib.aleo/NonExistent {
    fn main(x: u64) -> u64 {
        return 0;
    }
}
