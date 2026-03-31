// OK: Dynamic call with interface casts record to dyn record.
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
        return test.aleo::TokenOps@(target)::transfer(t as dyn record);
    }

    @noupgrade
    constructor() {}
}
