
program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public(receiver, amount); };
    }
}

final fn finalize_mint_private (public receiver: address, public amount: u64) {
    Mapping::set(account, receiver, amount);
}
