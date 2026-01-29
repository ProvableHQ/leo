
program test.aleo {
    fn matches(addr: address) -> bool {
        return self.caller == addr;
    }

    mapping balances: address => u128;
    mapping tokens: address => Token;

    mapping foo: Bar => Baz;
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

