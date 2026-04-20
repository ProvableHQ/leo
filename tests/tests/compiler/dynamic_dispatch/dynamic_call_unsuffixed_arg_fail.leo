// FAIL: unsuffixed integer literal as _dynamic_call argument without input_types annotation.
// The literal `1` has no expected type (input_types is absent), so type inference fails.
// Fix: annotate the call with _dynamic_call::[public u64, u64](...) or suffix the literal (1u64).
program test.aleo {
    fn main() -> u64 {
        return _dynamic_call::[u64]('a', 'aleo', 'b', 1);
    }
}
