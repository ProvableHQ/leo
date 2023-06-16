// The 'arc20' program.
program arc20.aleo {
    record Credential {
        owner: address,
        token_id: u64,
        total_supply: u64,
        // Minted amount 
        supplyed: u64,
    }

    record Token {
        owner: address,
        token_id: u64,
        amount: u64,
    }

    struct Order {
        caller: address,
        hold_token_id: u64,
        hold_token_amount: u64,

        expect_token_id: u64,
        expect_token_amount: u64,
    }
    struct Pair{
        addr: address,
        token_id: u64,
    }
    mapping exist: u64 => bool;
    mapping balances: Pair => u64;

    transition register(public token_id: u64, public total_supply: u64) -> Credential {
        let c: Credential = Credential {
            owner: self.caller,
            token_id: token_id,
            total_supply: total_supply,
            supplyed: 0u64,
        };
        return c then finalize(token_id);
    }
    finalize register(token_id: u64) {
        let flag: bool = Mapping::get_or_use(exist, token_id, false);
        assert_eq(flag, false);
        Mapping::set(exist, token_id, true);
    }

    transition mint(c: Credential, to: address, amount: u64) -> (Credential, Token) {
        assert(c.supplyed + amount <= c.total_supply);
        let new_c: Credential = Credential {
            owner: c.owner,
            token_id: c.token_id,
            total_supply: c.total_supply,
            supplyed: amount + c.supplyed
        };
        let t: Token = Token {
            owner: to,
            token_id: c.token_id,
            amount: amount
        };
        return (new_c, t);
    }
    transition transfer(t: Token, to: address, amount: u64) -> (Token, Token)  {
        let t1: Token = Token {
            owner: t.owner,
            token_id: t.token_id,
            amount: t.amount - amount,
        };
        let t2: Token = Token {
            owner: to,
            token_id: t.token_id,
            amount: amount,
        };
        return (t1, t2);
    }

    transition burn(t: Token) {}

    // User want to exchange token_id(1) for token_id(2)
    // Support exchange
    transition exchange(t: Token, expect_token_id: u64, expect_token_amount: u64) {

    }
}
