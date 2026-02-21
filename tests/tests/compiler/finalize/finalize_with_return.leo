program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public(receiver, amount); };
    }
    fn public_adder(public a: u8, public b: u8) -> Final {
        return final { finalize_public_adder(a, b); };
    }
}

final fn finalize_mint_public (public receiver: address, public amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
    return amount;
}

final fn finalize_public_adder(a: u8, b: u8) -> public u8 {
    return a + b;
}

