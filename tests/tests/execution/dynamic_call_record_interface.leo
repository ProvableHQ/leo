/*
seed = 123456789

[case]
program = "router.aleo"
function = "dispatch"
input = ["50u64"]
*/

interface TokenOps {
    record Token;
    fn double_balance(t: Token) -> Token;
}

program vault.aleo: TokenOps {
    record Token {
        owner: address,
        balance: u64,
    }

    fn double_balance(t: Token) -> Token {
        return Token { owner: t.owner, balance: t.balance * 2u64 };
    }

    // Mints a token owned by the transaction signer.
    fn mint(balance: u64) -> Token {
        return Token { owner: self.signer, balance };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import vault.aleo;

// router.aleo does not implement TokenOps — it is a pure dispatcher that routes
// dyn record tokens to any program that implements the interface at runtime.
program router.aleo {

    fn dispatch(balance: u64) -> u64 {
        let token: dyn record = vault.aleo::mint(balance) as dyn record;
        let result: dyn record = vault.aleo::TokenOps@('vault')::double_balance(token);
        return result.balance;
    }

    @noupgrade
    constructor() {}
}
