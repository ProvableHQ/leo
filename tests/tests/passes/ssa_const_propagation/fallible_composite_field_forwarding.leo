export struct Pair {
    checked: u8,
    payload: u8,
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn preserve_composite_base(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        let p: Pair = Pair { checked: diff, payload: 0u8 };
        return p.payload * 0u8;
    }

    fn keep_forwarding_plain_atom(x: u8) -> u8 {
        let p: Pair = Pair { checked: x, payload: 0u8 };
        return p.payload * 0u8;
    }
}
