// Test: interface defined in a submodule with record prototypes compiles without
// spurious signature-mismatch errors. The `Token` type in the interface's function
// prototypes must resolve to root scope rather than the submodule scope so that
// it matches the implementing program's `Token` record.

// --- library: token_lib --- //

// --- Next Module: itoken.leo --- //

interface TokenInterface {
    record Token;

    fn transfer(input: Token, recipient: address, amount: u128) -> (Token, Token);
    fn mint(recipient: address, amount: u128) -> Token;
}

// --- Next Program --- //

program test.aleo: token_lib::itoken::TokenInterface {
    record Token {
        owner: address,
        balance: u128,
    }

    fn transfer(input: Token, recipient: address, amount: u128) -> (Token, Token) {
        let to: Token = Token { owner: recipient, balance: amount };
        let change: Token = Token { owner: input.owner, balance: input.balance - amount };
        return (to, change);
    }

    fn mint(recipient: address, amount: u128) -> Token {
        return Token { owner: recipient, balance: amount };
    }

    @noupgrade
    constructor() {}
}
