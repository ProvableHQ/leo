/*
seed = 123456789

[case]
program = "test.aleo"
function = "mint_and_select"
input = ["true", "5u64", "50u64"]
[case]
program = "test.aleo"
function = "mint_and_select"
input = ["false", "5u64", "50u64"]
*/

program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint_and_select(s: bool, a: u64, b: u64) -> Token {
        let t1: Token = Token { owner: self.signer, amount: a };
        let t2: Token = Token { owner: self.signer, amount: b };
        return s ? t1 : t2;
    }

    @noupgrade
    constructor() {}
}
