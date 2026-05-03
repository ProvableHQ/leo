// ANCHOR: file
interface Transfer {
    record Token;
    fn transfer(input: Token, to: address, amount: u64) -> Token;
}

interface Balances {
    mapping balances: address => u64;
}

// MyToken requires everything from both Transfer and Balances
interface MyToken : Transfer + Balances {}

program my_token.aleo : MyToken {
    mapping balances: address => u64;

    record Token {
        owner: address,
        amount: u64,
    }

    fn transfer(input: Token, to: address, amount: u64) -> Token {
        return Token { owner: to, amount: input.amount - amount };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
