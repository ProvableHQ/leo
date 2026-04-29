// ANCHOR: interface
interface Transfer {
    record Token;
    fn transfer(input: Token, to: address, amount: u64) -> Token;
}
// ANCHOR_END: interface

// ANCHOR: program
program my_token.aleo : Transfer {
    mapping balances: address => u64;

    record Token {
        owner: address,
        balance: u64,
    }

    fn transfer(input: Token, to: address, amount: u64) -> Token {
        return Token { owner: to, balance: input.balance - amount };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: program
