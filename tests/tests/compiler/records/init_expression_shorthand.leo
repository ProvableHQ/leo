
program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }

    fn mint(owner: address, amount: u64) -> Token {
        return Token {
            owner,
            amount,
        };
    }

    fn main(x: address) -> u64 {
        let c: u64 = 1u64;
        let t: Token = Token { owner: x, amount: c};

        return c + c;
    }}
