/*
[case]
program = "main.aleo"
function = "get_val"
input = []
*/

// Test: instantiate a library generic struct, access a field, and return it.
// vec_lib::Slot::[7u32] has a `size` field. We construct it with size=7 and return it.
// Expected output: 7u32.

// --- library: vec_lib --- //

struct Slot::[N: u32] {
    size: u32,
}

// --- Next Program --- //

program main.aleo {
    fn get_val() -> u32 {
        let s: vec_lib::Slot::[7u32] = vec_lib::Slot::[7u32] { size: 7u32 };
        return s.size;
    }

    @noupgrade
    constructor() {}
}
