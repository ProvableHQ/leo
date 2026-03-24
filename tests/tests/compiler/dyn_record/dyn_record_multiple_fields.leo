program test.aleo {
    record Token {
        owner: address,
        balance: u64,
        memo: field,
    }

    fn main(t: Token) -> bool {
        let d: dyn record = t as dyn record;
        let b: u64 = d.balance;
        let m: field = d.memo;
        let o: address = d.owner;
        return b == 0u64;
    }
}
