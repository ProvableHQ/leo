program test.aleo {
    record Token {
        owner: address,
        amount: u64,
        nonce: u64,
    }

    // Copy `owner` and `nonce` from `base`, overriding only `amount`.
    fn update(base: Token, new_amount: u64) -> Token {
        return Token { amount: new_amount, ..base };
    }

    @noupgrade
    constructor() {}
}
