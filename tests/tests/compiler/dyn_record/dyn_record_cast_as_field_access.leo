program test.aleo {
    record Token {
        owner: address,
        balance: u64,
    }

    fn main(t: Token) -> u64 {
        let d: dyn record = t as dyn record;
        return d.balance as u64;
    }
}
