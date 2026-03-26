interface TokenInterface {
    record Token {
        owner: address,
        private amount: u64,
        ..
    }
}

program test.aleo: TokenInterface {
    record Token {
        owner: address,
        public amount: u64,
    }

    fn main() {}
}
