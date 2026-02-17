program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    transition mint(owner: address, amount: u64) -> Token {
        return Token { owner: owner, amount: amount };
    }
}
