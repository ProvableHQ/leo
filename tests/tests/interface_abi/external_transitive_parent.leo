// Test: implementing an external interface whose parent is in a third program
// emits ABIs for both the child and the grandparent.

interface Base {
    fn get() -> u64;
}

program lib_a.aleo {
    fn dummy() {}
}
// --- Next Program --- //
import lib_a.aleo;

interface Extended: lib_a.aleo::Base {
    fn set(v: u64) -> u64;
}

program lib_b.aleo {
    fn dummy() {}
}
// --- Next Program --- //
import lib_a.aleo;
import lib_b.aleo;

program test.aleo: lib_b.aleo::Extended {
    fn get() -> u64 {
        return 0u64;
    }

    fn set(v: u64) -> u64 {
        return v;
    }
}
