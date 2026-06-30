// A const-generic pair, parameterised by an integer tag `N`.
export struct Pair::[N: u32] {
    first: u32,
    second: u32,
}

// A const-generic helper that scales `x` by the type-level constant `N`.
export fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}
