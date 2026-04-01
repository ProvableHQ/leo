/*
[case]
program = "generic_struct_mod.aleo"
function = "main"
input = []
*/

// Test: a generic struct and its constructor function defined in a library submodule are
// instantiated at runtime and the resulting field value is correct.

// --- library: data_lib --- //

// --- Next Module: types.leo --- //
struct Buffer::[N: u32] {
    data: [u32; N],
}

fn fill(val: u32) -> Buffer::[3] {
    return Buffer::[3] { data: [val, val, val] };
}

// --- Next Program --- //

program generic_struct_mod.aleo {
    fn main() -> u32 {
        let b: data_lib::types::Buffer::[3] = data_lib::types::fill(7u32);
        return b.data[0u32];  // 7
    }

    @noupgrade
    constructor() {}
}
