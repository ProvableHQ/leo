program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final {
            let new_balance: u64 = finalize_mint_public(receiver, amount);
            Mapping::set(account, receiver, new_balance);
        };
    }

    fn public_adder(public a: u8, public b: u8) -> Final {
        return final {
            let sum: u8 = finalize_public_adder(a, b);
            Mapping::set(values, 0u8, sum);
        };
    }
}

final fn finalize_mint_public(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    return current + amount;
}

final fn finalize_public_adder(a: u8, b: u8) -> u8 {
    return a + b;
}
