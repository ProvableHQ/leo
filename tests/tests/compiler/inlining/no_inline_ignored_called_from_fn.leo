@no_inline
export fn inner(x: u32) -> u32 {
    return x + 1u32;
}

export fn outer(x: u32) -> u32 {
    return inner(x);
}

program test.aleo {
    fn main(x: u32) -> u32 {
        return outer(x);
    }

    @noupgrade
    constructor() {}
}
