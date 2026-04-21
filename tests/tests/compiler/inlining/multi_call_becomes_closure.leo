// Test: a fn called more than once becomes a `closure` in Aleo bytecode.
// The call_count == 1 heuristic does not fire (count == 2), and no mandatory
// condition applies, so the function is NOT inlined.  Each call site emits
// a `call sum_of` in the output.
//
// Note: the function is named `sum_of` rather than `add` to avoid a name clash
// with the Aleo `add` instruction, which would cause the Aleo parser to reject
// `closure add:`.

fn sum_of(a: u32, b: u32) -> u32 {
    return a + b;
}

program test.aleo {
    fn main(x: u32, y: u32, z: u32) -> (u32, u32) {
        let first: u32 = sum_of(x, y);
        let second: u32 = sum_of(y, z);
        return (first, second);
    }
}
