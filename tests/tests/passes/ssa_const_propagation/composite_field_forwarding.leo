program test.aleo {
    struct Pair {
        a: u32,
        b: u32,
    }

    // Composite literals with atom-valued fields are tracked; subsequent
    // `s.a` / `s.b` accesses are forwarded to the stored atom. The
    // `Pair { ... }` binding becomes dead — DCE removes it in a later pass —
    // but the tracking itself is visible here as the member accesses being
    // replaced by the original paths/literals.
    fn literal_fields() -> u32 {
        let s: Pair = Pair { a: 1u32, b: 2u32 };
        return s.a + s.b;
    }

    fn path_fields(p: u32, q: u32) -> u32 {
        let s: Pair = Pair { a: p, b: q };
        return s.a + s.b;
    }
}
