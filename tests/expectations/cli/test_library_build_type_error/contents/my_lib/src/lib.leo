// The `u32 + bool` expression must be rejected by type checking.
export fn bad_add(x: u32) -> u32 {
    return x + true;
}
