interface TokenInterface {
    record Token { }
}

program test.aleo : TokenInterface {
    record Token {
        owner: address,
    }
    fn main() {}
}
