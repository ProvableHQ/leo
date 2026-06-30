// Test: program defines a struct named Token but the interface requires a record
// named Token. The function signature check passes (name-based matching accepts Token
// in both cases) but the record-presence check correctly fires a missing_interface_record
// error because lookup_record finds no record with that name.

export interface IToken {
    record Token {
        owner: address,
        ..
    }
    fn mint(owner: address) -> Token;
}

export struct Token {
    owner: address,
    amount: u64,
}

program test.aleo: IToken {
    @noupgrade
    constructor() {}

    fn mint(owner: address) -> Token {
        return Token { owner: owner, amount: 0u64 };
    }
}
