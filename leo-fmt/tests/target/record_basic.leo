program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint(owner: address, amount: u64) -> Token {
        return Token { owner: owner, amount: amount };
    }
}
