program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn select(s: bool, a: Token, b: Token) -> Token {
        return s ? a : b;
    }
}
