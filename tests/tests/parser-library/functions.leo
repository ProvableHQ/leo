fn simple(a: u32, b: u32) -> u32 {
    return a + b;
}

fn no_args() -> u32 {
    return 42u32;
}

fn generic::[N: u32](x: u32) -> u32 {
    return x * N;
}

fn multi_generic::[A: u32, B: u32](x: u32) -> u32 {
    return x * A + B;
}

fn calls_other(x: u32) -> u32 {
    return simple(x, x);
}
