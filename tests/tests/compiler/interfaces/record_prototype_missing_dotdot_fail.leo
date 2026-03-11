interface TokenInterface {
    record Token {
        owner: address
    }
}

program test.aleo : TokenInterface {
    record Token {
        owner: address,
        amount: u64,
    }
    fn main() {}
}
