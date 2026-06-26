/*
seed = 123456789

[case]
program = "test.aleo"
function = "make_and_update"
input = ["50u64"]
[case]
program = "test.aleo"
function = "make_and_update"
input = ["0u64"]
*/

program test.aleo {
    record Token {
        owner: address,
        amount: u64,
        nonce: u64,
    }

    // Build a base record from the signer, then derive a new record that copies
    // `owner` and `nonce` from the base while overriding only `amount`.
    fn make_and_update(new_amount: u64) -> (Token, Token) {
        let base: Token = Token { owner: self.signer, amount: 100u64, nonce: 7u64 };
        let updated: Token = Token { amount: new_amount, ..base };
        return (base, updated);
    }

    @noupgrade
    constructor() {}
}
