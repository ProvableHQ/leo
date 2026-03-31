program my_program.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    // Mints a new Token record owned by the given address.
    fn mint(owner: address, amount: u64) -> Token {
        return Token { owner, amount };
    }

    // Returns twice the given amount.
    fn double_amount(x: u64) -> u64 {
        return x * 2u64;
    }

    @noupgrade
    constructor() {}
}
