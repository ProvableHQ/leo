
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
    Mapping::set(tokens, addr, amount);
    tokens.set(addr, amount);
    Mapping::set(amounts, 1u8, amount);
    amounts.set(1u8, amount);
    Mapping::set(amounts, addr, 1u8);
    amounts.set(addr, 1u8);
    Mapping::set(tokens, addr, Token { owner_addr: addr, amount: 1u8 });
    tokens.set(addr, Token { owner_addr: addr, amount: 1u8 });
}
