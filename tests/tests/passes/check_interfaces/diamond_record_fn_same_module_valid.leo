// Test: diamond interface inheritance with record types in function signatures, all
// interfaces defined in the same program scope (no library module). This exercises
// the inheritance graph + prototypes_match + inheritance_record_names code path.
//
// IDiamond: ILeft + IRight, both inheriting from IBase. flatten_interface(IDiamond)
// encounters issue() from both ILeft and IRight (both inherited from IBase) and calls
// prototypes_match with inheritance_record_names = {Token} to verify they are
// compatible. The Token record type is matched by name, not by fully-qualified path.

interface IBase {
    record Token {
        owner: address,
        ..
    }
    fn issue(owner: address) -> Token;
}

interface ILeft: IBase {
    fn split(t: Token) -> (Token, Token);
}

interface IRight: IBase {
    fn owner_of(t: Token) -> address;
}

interface IDiamond: ILeft + IRight {}

program test.aleo: IDiamond {
    record Token {
        owner: address,
        amount: u64,
    }

    fn issue(owner: address) -> Token {
        return Token { owner: owner, amount: 0u64 };
    }

    fn split(t: Token) -> (Token, Token) {
        let half: u64 = t.amount / 2u64;
        let a: Token = Token { owner: t.owner, amount: half };
        let b: Token = Token { owner: t.owner, amount: half };
        return (a, b);
    }

    fn owner_of(t: Token) -> address {
        return t.owner;
    }
}
