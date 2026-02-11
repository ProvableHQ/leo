
struct Token {
    owner_addr: address,
    amount: u128,
}

program test.aleo {
    mapping amounts: address => u128;
    mapping tokens: address => Token;

    fn decrease_self(amount: u128) -> Final {
        let caller = self.caller;
        return final { finalize_decrease_self(caller, amount); };
    }
}

final fn finalize_decrease_self(addr: address, amount: u128) {
    Mapping::get_or_use(tokens, addr, amount);
    tokens.get_or_use(addr, amount);
    Mapping::get_or_use(amounts, 1u8, amount);
    amounts.get_or_use(1u8, amount);
    Mapping::get_or_use(amounts, addr, 1u8);
    amounts.get_or_use(addr, 1u8);
    Mapping::get_or_use(tokens, addr, Token { owner_addr: addr, amount: 1u8 });
    tokens.get_or_use(addr, Token { owner_addr: addr, amount: 1u8 });
}
