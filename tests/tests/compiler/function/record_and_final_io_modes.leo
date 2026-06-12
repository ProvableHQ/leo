// PASS: records and `Final`s WITHOUT a visibility mode are accepted on entry points, and a mode on a
// non-record / non-`Final` type (e.g. a `struct`) is still accepted. This guards against the
// record/`Final` mode check over-firing on the types it should leave alone.
struct Pair {
    a: u32,
    b: u32,
}

program test.aleo {
    mapping Yo: u32 => u32;

    record Token {
        owner: address,
        amount: u64,
    }

    // Record I/O without a mode: OK.
    fn rec_io(t: Token) -> Token { return t; }

    // A struct (not a record) may carry a visibility mode: OK.
    fn struct_io(public p: Pair) -> public Pair { return p; }

    // `dyn record` I/O without a mode: OK.
    fn dyn_io(t: dyn record) -> dyn record { return t; }

    // `Final` output without a mode: OK.
    fn fut_io() -> Final { return final { fin(1u32, 1u32); }; }
}

final fn fin(a: u32, b: u32) {
    Mapping::set(Yo, a, b);
}
