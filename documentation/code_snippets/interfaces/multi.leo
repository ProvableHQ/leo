// ANCHOR: program
interface Transfer {
    record Token;
    fn transfer(input: Token, to: address, amount: u64) -> Token;
}

interface Pausable {
    mapping paused: address => bool;
    fn pause() -> (bool, Final);
}

// my_token.aleo must satisfy both Transfer and Pausable
program my_token.aleo : Transfer + Pausable {
    mapping paused: address => bool;

    record Token {
        owner: address,
        balance: u64,
    }

    fn transfer(input: Token, to: address, amount: u64) -> Token {
        return Token { owner: to, balance: input.balance - amount };
    }

    fn pause() -> (bool, Final) {
        let caller: address = self.caller;
        return (true, final {
            Mapping::set(paused, caller, true);
        });
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: program
