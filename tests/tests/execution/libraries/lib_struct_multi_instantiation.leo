/*
[case]
program = "main.aleo"
function = "sum_tags"
input = []
*/

// Test: two distinct monomorphizations of the same library generic struct.
// Tag::[1u32] and Tag::[2u32] are independent types; their `id` fields are summed.
// Expected output: 10u32 + 20u32 = 30u32.

// --- library: tag_lib --- //

struct Tag::[N: u32] {
    id: u32,
}

// --- Next Program --- //

program main.aleo {
    fn sum_tags() -> u32 {
        let a: tag_lib::Tag::[1u32] = tag_lib::Tag::[1u32] { id: 10u32 };
        let b: tag_lib::Tag::[2u32] = tag_lib::Tag::[2u32] { id: 20u32 };
        return a.id + b.id;
    }

    @noupgrade
    constructor() {}
}
