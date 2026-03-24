/*
seed = 123456789

[case]
program = "test.aleo"
function = "get_balance"
input = ["aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs", "100u64", "42field"]
*/

program test.aleo {
    record Token {
        owner: address,
        balance: u64,
        memo: field,
    }

    fn get_balance(owner: address, balance: u64, memo: field) -> u64 {
        let t: Token = Token { owner, balance, memo };
        let d: dyn record = t as dyn record;
        let b: u64 = d.balance;
        return b;
    }

    @noupgrade
    constructor() {}
}
