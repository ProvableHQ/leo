export interface TokenA {
    record Token {
        owner: address,
        amount: u64,
        ..
    }
}

export interface TokenB {
    record Token {
        owner: address,
        amount: u32,
        ..
    }
}

export interface Combined: TokenA + TokenB {
}

program test.aleo: Combined {
    fn main() {}

    @noupgrade
    constructor() {}
}
