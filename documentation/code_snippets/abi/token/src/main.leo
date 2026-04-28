program token.aleo {
    mapping account: address => u64;

    record Token {
        owner: address,
        amount: u64,
    }

    fn mint_public(
        public receiver: address,
        public amount: u64
    ) -> Final {
        return final {
            let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
            Mapping::set(account, receiver, current + amount);
        };
    }

    fn mint_private(receiver: address, amount: u64) -> Token {
        return Token { owner: receiver, amount };
    }

    fn transfer_private(token: Token, receiver: address) -> Token {
        return Token { owner: receiver, amount: token.amount };
    }

    @noupgrade
    constructor() {}
}
