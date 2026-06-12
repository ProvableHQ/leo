struct Foo {
    x: u32,
}

const X: u32 = 4u32; // trailing on const

const Y: bool = true; /* block trailing */

program test.aleo {
    @noupgrade
    constructor() {}
    mapping balances: address => u64; // trailing on mapping

    fn main(f: Foo) -> u32 {
        return f.x + X;
    }
    // after fn
}
