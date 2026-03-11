interface TokenInterface {
    record Token {
        owner: u64,
        ..
    }
}

program test.aleo : TokenInterface {
    record Token {
        owner: address,
        amount: u64,
    }
    fn main() {}
}
