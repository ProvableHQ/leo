program test.aleo {
    record Token {
        owner: address,
        balance: u64,
        memo: field,
    }

    fn make_dynamic(t: Token) -> dyn {
        let dynamic = t as dyn;
        return dynamic;
    }

    fn get_balance(r: dyn) -> u64 {
        let balance: u64 = r.balance;
        return balance;
    }

    fn get_owner(r: dyn) -> address {
        return r.owner;
    }

    fn get_memo_typed(r: dyn) -> field {
        let memo: field = r.memo;
        return memo;
    }
}
