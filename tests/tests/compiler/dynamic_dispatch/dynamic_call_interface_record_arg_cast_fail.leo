// FAIL: A non-implementing caller passes a concrete record to a dynamic call.
// Record arguments to interface dynamic calls must always be dyn record.
interface TokenOps {
    record Token;
    fn transfer(t: Token) -> Token;
}

program vault.aleo: TokenOps {
    record Token {
        owner: address,
        amount: u64,
    }

    fn transfer(t: Token) -> Token {
        return t;
    }

    fn mint(owner: address, amount: u64) -> Token {
        return Token { owner, amount };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import vault.aleo;

program caller.aleo {
    fn route(target: field, owner: address, amount: u64) -> dyn record {
        let t: vault.aleo::Token = vault.aleo::mint(owner, amount);
        return vault.aleo::TokenOps@(target)::transfer(t);
    }

    @noupgrade
    constructor() {}
}
