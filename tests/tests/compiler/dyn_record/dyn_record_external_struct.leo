struct Metadata {
    amount: u64,
}

program token.aleo {
    record Token {
        owner: address,
        meta: Metadata,
    }

    fn mint(m: Metadata) -> Token {
        return Token { owner: self.signer, meta: m };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import token.aleo;
program test.aleo {
    fn main(t: token.aleo::Token) -> u64 {
        let d: dyn record = t as dyn record;
        let m: token.aleo::Metadata = d.meta;
        return m.amount;
    }
}
