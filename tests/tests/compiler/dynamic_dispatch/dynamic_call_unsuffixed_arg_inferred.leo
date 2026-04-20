// PASS: unsuffixed integer literal as _dynamic_call argument, inferred from explicit input_types.
// The literal `1` resolves to u64 because input_types provides the expected type.
program test.aleo {
    fn main() -> u64 {
        return _dynamic_call::[public u64, u64]('a', 'aleo', 'b', 1);
    }
}
