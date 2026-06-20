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
}
