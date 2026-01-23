program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }
    fn public_adder(public a: u8, public b: u8) -> Future {
        return finalize_public_adder(a, b);
    }
}

onchain fn finalize_mint_public (public receiver: address, public amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
    return amount;
}

onchain fn finalize_public_adder(a: u8, b: u8) -> public u8 {
    return a + b;
}

