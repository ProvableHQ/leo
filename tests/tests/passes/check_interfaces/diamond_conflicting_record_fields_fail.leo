// Test: diamond inheritance where two parent interfaces declare record Token with
// incompatible field types. ILeft requires amount: u64 while IRight requires
// amount: u128. flatten_interface must detect the conflict via record_fields_compatible
// and emit conflicting_record_field when building ICombined.

interface ILeft {
    record Token {
        owner: address,
        amount: u64,
        ..
    }
    fn transfer(t: Token) -> Token;
}

interface IRight {
    record Token {
        owner: address,
        amount: u128,
        ..
    }
    fn balance(t: Token) -> u128;
}

interface ICombined: ILeft + IRight {}

program test.aleo: ICombined {
    record Token {
        owner: address,
        amount: u64,
    }

    fn transfer(t: Token) -> Token {
        return t;
    }

    fn balance(t: Token) -> u128 {
        return t.amount as u128;
    }
}
