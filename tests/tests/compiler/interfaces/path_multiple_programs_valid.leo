interface InterfaceA {
    fn fn_a() -> u64;
}

program lib_a.aleo {
    fn dummy() {}
}

// --- Next Program --- //

interface InterfaceB {
    fn fn_b() -> u64;
}

program lib_b.aleo {
    fn dummy() {}
}

// --- Next Program --- //

import lib_a.aleo;
import lib_b.aleo;

program test.aleo : lib_a.aleo/InterfaceA + lib_b.aleo/InterfaceB {
    fn fn_a() -> u64 {
        return 1u64;
    }

    fn fn_b() -> u64 {
        return 2u64;
    }

    fn main(x: u64) -> u64 {
        return 0;
    }
}
