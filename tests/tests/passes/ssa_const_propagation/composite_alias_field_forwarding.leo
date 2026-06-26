struct Pair {
    a: u32,
    b: u32,
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn alias_field(p: u32, q: u32) -> u32 {
        let s: Pair = Pair { a: p, b: q };
        let alias: Pair = s;
        return alias.a + alias.b;
    }

    fn alias_chain(p: u32, q: u32) -> u32 {
        let s: Pair = Pair { a: p, b: q };
        let alias1: Pair = s;
        let alias2: Pair = alias1;
        return alias2.a + alias2.b;
    }

    fn fallible_field(p: u32, q: u32) -> u32 {
        let s: Pair = Pair { a: p / q, b: q };
        let alias: Pair = s;
        return alias.a + alias.b;
    }
}
