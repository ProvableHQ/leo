// FAIL: Dynamic call with interface passes concrete record instead of dyn record.
interface TokenOps {
    fn transfer(t: Token) -> Token;
}

program test.aleo: TokenOps {
    record Token {
        owner: address,
        amount: u64,
    }

    fn transfer(t: Token) -> Token {
        return t;
    }

    fn main(target: field, t: Token) -> dyn record {
        return test.aleo::TokenOps@(target)::transfer(t);
    }

    @noupgrade
    constructor() {}
}
