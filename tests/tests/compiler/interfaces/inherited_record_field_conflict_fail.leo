interface TokenA {
    record Token {
        owner: address,
        amount: u64,
        ..
    }
}

interface TokenB {
    record Token {
        owner: address,
        amount: u32,
        ..
    }
}

interface Combined : TokenA + TokenB {}

program test.aleo : Combined {
    fn main() {}
}
