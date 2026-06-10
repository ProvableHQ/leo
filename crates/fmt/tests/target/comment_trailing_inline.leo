struct Foo {
    x: u32,
}

program test.aleo {
    const X: u32 = 4u32; // trailing on const

    mapping balances: address => u64; // trailing on mapping

    const Y: bool = true; /* block trailing */

    fn main(f: Foo) -> u32 {
        return f.x + X;
    }
    // after fn
}
