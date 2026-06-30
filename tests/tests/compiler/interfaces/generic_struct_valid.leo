// Test: interface function prototype using a const-generic struct compiles end-to-end.
// The implementing program must match the monomorphized signature.

export struct Slot::[N: u32] {
    val: u32,
}

export interface Storer {
    fn store(s: Slot::[8u32]) -> u32;
}

program test.aleo: Storer {
    fn store(s: Slot::[8u32]) -> u32 {
        return s.val;
    }

    fn main(s: Slot::[8u32]) -> u32 {
        return s.val;
    }

    @noupgrade
    constructor() {}
}
