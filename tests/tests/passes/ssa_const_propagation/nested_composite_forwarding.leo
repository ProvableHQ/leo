struct Inner {
    v: u32,
}

struct Outer {
    inner: Inner,
}

program test.aleo {
    @noupgrade
    constructor() {}
    // Nested composites: SSA lowers `o.inner.v` into a pair of accesses
    // (`$t = o.inner; $t.v`). The first access forwards to `Path(i)`, then
    // simple alias forwarding lets the second access see through `$t` and
    // forward `i.v`.
    fn main() -> u32 {
        let i: Inner = Inner { v: 42u32 };
        let o: Outer = Outer { inner: i };
        return o.inner.v;
    }
}
