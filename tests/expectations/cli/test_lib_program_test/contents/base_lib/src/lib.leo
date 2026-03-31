// Returns x * 2.
fn double(x: u32) -> u32 {
    return x * 2u32;
}

// Returns x * N (generic version).
fn multiply::[N: u32](x: u32) -> u32 {
    return x * N;
}
