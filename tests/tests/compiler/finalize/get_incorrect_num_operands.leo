
program test.aleo {
    record Token {
        owner: address,
        amount: u128,
    }

    mapping amounts: address => u128;
    mapping tokens: address => Token;

    async fn decrease_self(amount: u128) -> Future {
        return finalize_decrease_self(self.caller, amount);
    }
}

async fn finalize_decrease_self(addr: address, amount: u128) {
    Mapping::get(tokens, true, true);
    tokens.get(true, true);
    Mapping::get(amounts);
    amounts.get();
}
