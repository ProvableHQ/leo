program test.aleo {
    fn main(x: u32) -> u32 {
        return identity(x);
    }

    @noupgrade
    constructor() {}
}

fn identity(x: u32) -> u32 {
    return x;
}

