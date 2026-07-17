export struct Pair {
    a: u32,
    b: u32,
}

// Returns the sum of the pair's fields.
export fn sum(p: Pair) -> u32 {
    return p.a + p.b;
}
