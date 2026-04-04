// Test: record prototype declared only in a grandparent interface (three-level chain).
// IBase declares record Token and fn mint. ITransfer (child of IBase) adds fn transfer.
// IFull (child of ITransfer) adds fn burn. The implementing program must satisfy all
// three functions and the Token record even though they come from different levels.
// Verifies that record_names and flatten_interface collect records transitively.

interface IBase {
    record Token {
        owner: address,
        ..
    }
    fn mint(owner: address) -> Token;
}

interface ITransfer: IBase {
    fn transfer(t: Token, to: address) -> Token;
}

interface IFull: ITransfer {
    fn burn(t: Token) -> address;
}

program test.aleo: IFull {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint(owner: address) -> Token {
        return Token { owner: owner, amount: 0u64 };
    }

    fn transfer(t: Token, to: address) -> Token {
        return Token { owner: to, amount: t.amount };
    }

    fn burn(t: Token) -> address {
        return t.owner;
    }
}
