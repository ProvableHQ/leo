
program test.aleo {
    record Token {
        // The token amount.
        amount: u64,
        // The token owner.
        owner: address,
    }

    fn main(a: u64, b:u64) -> Token {
        return Token { amount: a + b, owner: self.signer };
    }
}
