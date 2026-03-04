interface TokenInterface {
    record Token {
        owner: address,
        amount: u64,
        ..
    }
}

program test.aleo : TokenInterface {
    record Token {
        owner: address,
        amount: u64,
        metadata: u32,
    }
    fn main() {}
}
