// ANCHOR: file
// 4.0
program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint(public amount: u64) -> Token {
        return Token { owner: self.caller, amount: amount };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
