// Negative test: `@no_inline` on a function with const parameters is a compile
// error.  The type checker rejects this combination outright (ETYC0372143) —
// there is nothing to ignore because generic functions cannot be closures.

@no_inline
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

program test.aleo {
    fn main(x: u32) -> u32 {
        return scale::[3u32](x);
    }
}
