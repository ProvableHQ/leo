program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }

    @noupgrade
    constructor() {}
}

export struct Token { // This struct cannot have the same name as the record defined above it.
    x: u32,
}

export fn main(zz: bool) -> bool {
    return true;
}
