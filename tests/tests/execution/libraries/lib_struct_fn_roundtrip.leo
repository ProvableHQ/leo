/*
[case]
program = "main.aleo"
function = "roundtrip"
input = ["42u32"]
*/

// Test: pass a library struct through a function call and back.
// scale doubles the `val` field of a Wrap::[3u32] struct.
// Input: 42u32. Expected output: the `val` of the returned struct = 84u32.

// --- library: wrap_lib --- //

struct Wrap::[N: u32] {
    val: u32,
}

// --- Next Program --- //

fn scale(w: wrap_lib::Wrap::[3u32]) -> wrap_lib::Wrap::[3u32] {
    return wrap_lib::Wrap::[3u32] { val: w.val * 2u32 };
}

program main.aleo {
    fn roundtrip(x: u32) -> u32 {
        let w: wrap_lib::Wrap::[3u32] = wrap_lib::Wrap::[3u32] { val: x };
        let w2: wrap_lib::Wrap::[3u32] = scale(w);
        return w2.val;
    }

    @noupgrade
    constructor() {}
}
