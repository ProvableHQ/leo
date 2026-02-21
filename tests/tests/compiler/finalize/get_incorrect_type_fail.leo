
program test.aleo {
    record Token {
        owner: address,
        amount: u128,
    }

    mapping amounts: address => u128;
    mapping tokens: address => Token;

    fn decrease_self(amount: u128) -> Final {
        let caller = self.caller;
        return final { finalize_decrease_self(caller, amount); };
    }
}

final fn finalize_decrease_self(addr: address, amount: u128) {
    Mapping::get(tokens, true);
    tokens.get(true);
    Mapping::get(amounts, 1u8);
    amounts.get(1u8);
}
