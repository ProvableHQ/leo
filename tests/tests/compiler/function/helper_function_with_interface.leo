program test.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    constructor() {}
}

export struct Board {
    foo: u8,
}

export fn win(b: Board, p: u8) -> bool {
    return false & true;
}
