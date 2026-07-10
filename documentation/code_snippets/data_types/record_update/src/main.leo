// ANCHOR: file
program record_update.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    // Copy `owner` from `base`, overriding only `amount`.
    fn top_up(base: Token) -> Token {
        return Token { amount: 100u64, ..base };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
