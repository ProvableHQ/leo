
program test.aleo {
    record Token {
        owner: address,
        amount: u128,
    }

    mapping amounts: address => u128;
    mapping tokens: address => Token;

    fn decrease_self(amount: u128) -> Final {
        return final { finalize_decrease_self(self.caller, amount); };
    }
}

final fn finalize_decrease_self(addr: address, amount: u128) {
    Mapping::set(tokens, addr, amount, 1u128);
    tokens.set(addr, amount, 1u128);
    Mapping::set(amounts, 1u8);
    amounts.set(1u8);
    Mapping::set(amounts);
    amounts.set();
}
