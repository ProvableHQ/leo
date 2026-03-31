// Test: generic functions and generic structs parse correctly as module items.

const FACTOR: u32 = 5u32;

struct Buffer::[N: u32] { data: [u32; N] }

fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

fn multi_generic::[A: u32, B: u32](x: u32) -> u32 {
    return x * A + B;
}
