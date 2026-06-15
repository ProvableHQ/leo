program test.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    constructor() {}
}

struct Board {
    foo: u8,
}

fn _win(_b: Board, _p: u8) -> bool {
    return false & true;
}
