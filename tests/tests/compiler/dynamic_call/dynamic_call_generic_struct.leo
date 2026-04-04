// Test: dynamic call through an interface whose function prototype uses a const-generic struct.
struct Slot::[N: u32] {
    val: u32,
}

interface Storer {
    fn store(s: Slot::[8u32]) -> u32;
}

program test.aleo: Storer {
    fn store(s: Slot::[8u32]) -> u32 {
        return s.val;
    }

    fn main(target: field, s: Slot::[8u32]) -> u32 {
        return Storer@(target)::store(s);
    }
}
