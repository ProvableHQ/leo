program test.aleo {
    struct Inner {
        v: u32,
    }

    struct Outer {
        inner: Inner,
    }

    // Nested composites: SSA lowers `o.inner.v` into a pair of accesses
    // (`$t = o.inner; $t.v`). The first access forwards to the tracked
    // atom for `o.inner` (`Path(i)`), but the second access sits on the
    // alias `$t` rather than on `i`, and alias-to-composite propagation
    // is intentionally out of scope for this pass — so the second hop
    // remains a member access and is left for DCE + later iterations to
    // collapse. This test pins down the current single-hop behavior.
    fn main() -> u32 {
        let i: Inner = Inner { v: 42u32 };
        let o: Outer = Outer { inner: i };
        return o.inner.v;
    }
}
