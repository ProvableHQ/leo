// Test: implementing a function with a different const argument than what the
// interface requires is a signature mismatch and must be rejected.

struct Slot::[N: u32] {
    val: u32,
}

interface Storer {
    fn store(s: Slot::[8u32]) -> u32;
}

// The implementing function uses Slot::[16u32] instead of the required Slot::[8u32].
program test.aleo: Storer {
    fn store(s: Slot::[16u32]) -> u32 {
        return s.val;
    }

    fn main(s: Slot::[16u32]) -> u32 {
        return s.val;
    }
}
