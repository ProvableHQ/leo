// Returns x * 2.
export fn double(x: u32) -> u32 {
    return x * 2u32;
}

// Returns x * N (generic version).
export fn multiply::[N: u32](x: u32) -> u32 {
    return x * N;
}
