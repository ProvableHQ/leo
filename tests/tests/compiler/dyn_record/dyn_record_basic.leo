program test.aleo {
    record Token {
        owner: address,
        balance: u64,
        memo: field,
    }

    fn make_dynamic(t: Token) -> dyn record {
        let dynamic = t as dyn record;
        return dynamic;
    }

    fn get_balance(r: dyn record) -> u64 {
        let balance: u64 = r.balance;
        return balance;
    }

    fn get_owner(r: dyn record) -> address {
        return r.owner;
    }

    fn get_memo_typed(r: dyn record) -> field {
        let memo: field = r.memo;
        return memo;
    }
}
