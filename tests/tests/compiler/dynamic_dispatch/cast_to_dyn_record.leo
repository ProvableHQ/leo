program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn main(t: Token) -> dyn record {
        return t as dyn record;
    }
}
