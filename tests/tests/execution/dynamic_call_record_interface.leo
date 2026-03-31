/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs", "50u64"]
*/

interface TokenOps {
    fn double_balance(t: Token) -> Token;
}

program token_impl.aleo: TokenOps {
    record Token {
        owner: address,
        balance: u64,
    }

    fn double_balance(t: Token) -> Token {
        return Token { owner: t.owner, balance: t.balance * 2u64 };
    }

    fn get_initial(balance: u64) -> Token {
        return Token { owner: self.signer, balance };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import token_impl.aleo;

program caller.aleo {

    fn main(owner: address, balance: u64) -> u64 {
        let t: token_impl.aleo::Token = token_impl.aleo::get_initial(balance);
        let result: dyn record = token_impl.aleo::TokenOps@('token_impl')::double_balance(t as dyn record);
        let b: u64 = result.balance;
        return b;
    }

    @noupgrade
    constructor() {}
}
