program test.aleo {
    fn main(public a: u128) -> public u128 {
        return f(a);
    }

    @noupgrade
    constructor() {}
}

export final fn f(x: u128) -> u128 {
    return block.height == 0u32 ? 0u128 : x;
}
