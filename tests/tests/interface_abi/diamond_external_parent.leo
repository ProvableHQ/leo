// Test: diamond inheritance across programs - both paths lead to the same
// grandparent which should be emitted exactly once.

interface Root {
    fn root_fn() -> u64;
}

program root.aleo {
    fn dummy() {}
}
// --- Next Program --- //
import root.aleo;

interface Left: root.aleo::Root {
    fn left_fn() -> u64;
}

program left.aleo {
    fn dummy() {}
}
// --- Next Program --- //
import root.aleo;

interface Right: root.aleo::Root {
    fn right_fn() -> u64;
}

program right.aleo {
    fn dummy() {}
}
// --- Next Program --- //
import root.aleo;
import left.aleo;
import right.aleo;

program test.aleo: left.aleo::Left + right.aleo::Right {
    fn root_fn() -> u64 {
        return 0u64;
    }

    fn left_fn() -> u64 {
        return 1u64;
    }

    fn right_fn() -> u64 {
        return 2u64;
    }
}
