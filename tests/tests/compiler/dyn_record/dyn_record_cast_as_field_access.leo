program test.aleo {
    record Token {
        owner: address,
        balance: u64,
    }

    fn main(t: Token) -> u64 {
        let d: dyn = t as dyn;
        return d.balance as u64;
    }
}
