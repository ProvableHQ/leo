/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["97field", "1868917857field", "102field", "aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs", "50u64"]
*/

program a.aleo {
    record Token {
        owner: address,
        balance: u64,
    }

    fn f(t: Token) -> Token {
        return Token { owner: t.owner, balance: t.balance + t.balance };
    }

    fn get_initial(balance: u64) -> Token {
        return Token { owner: self.signer, balance };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import a.aleo;

program caller.aleo {

    fn main(prog: field, net: field, func: field, owner: address, balance: u64) -> u64 {
        let t: a.aleo::Token = a.aleo::get_initial(balance);
        let result: dyn record = _dynamic_call::[dyn record, dyn record](prog, net, func, t as dyn record);
        let new_balance: u64 = result.balance;
        return new_balance;
    }

    @noupgrade
    constructor() {}
}
