program test.aleo {
    fn matches(addr: address) -> bool {
        return std::ctx::caller() == addr;
    }

    mapping balances: address => u128;
    mapping tokens: address => Token;

    mapping foo: Bar => Baz;

    @noupgrade
    constructor() {}
}

struct Token {
    Owner: address,
    balance: u128,
}

struct Bar {
    a: u128,
}

struct Baz {
    a: u128,
}
